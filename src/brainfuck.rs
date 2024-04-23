use std::{collections::HashMap, io::Read};

#[derive(Debug)]
enum Token {
    Incptr,
    Decptr,
    Incbyte,
    Decbyte,
    Outbyte,
    Inbyte,
    Forward,
    Backward,
}

pub struct Interpreter {
    ip: usize,                    // Instruction Pointer
    dp: usize,                    // Data Pointer
    cells: Vec<isize>,            // Vector of bytes
    insns: Vec<Token>,            // instrutions are list of Tokens
    jumps: HashMap<usize, usize>, // Keep track of jumps (forward and backwards)
}

// Our function just return Err(()) and print the error
type Result<T> = std::result::Result<T, ()>;

impl Interpreter {
    pub fn new(code: &str) -> Result<Self> {
        let mut insns: Vec<Token> = Vec::new();

        code.chars().for_each(|c| match c {
            '>' => insns.push(Token::Incptr),
            '<' => insns.push(Token::Decptr),
            '+' => insns.push(Token::Incbyte),
            '-' => insns.push(Token::Decbyte),
            '.' => insns.push(Token::Outbyte),
            ',' => insns.push(Token::Inbyte),
            '[' => insns.push(Token::Forward),
            ']' => insns.push(Token::Backward),
            _ => {}
        });

        // Let's keep track of jumps in a second pass.
        let mut jumps_loc: Vec<usize> = Vec::new(); // keep track of open brackets position
        let mut jumps = HashMap::new();
        for (i, c) in insns.iter().enumerate() {
            match c {
                Token::Forward => {
                    jumps_loc.push(i);
                }
                Token::Backward => {
                    match jumps_loc.pop() {
                        None => {
                            eprintln!("unbalanced brackets");
                            return Err(());
                        }
                        Some(forward_ip) => {
                            // We add both jumps
                            jumps.insert(i, forward_ip);
                            jumps.insert(forward_ip, i);
                        }
                    };
                }
                _ => { // Nothing to do}
                }
            }
        }

        if !jumps_loc.is_empty() {
            eprintln!("Missing closed brackets");
            return Err(());
        }

        Ok(Self {
            ip: 0,
            dp: 0,
            insns,
            cells: vec![0; 1024],
            jumps,
        })
    }

    fn interpreter_state(&self) {
        println!("-----------------------------------");
        println!("Next instruction: {:?}", self.insns[self.ip]);
        println!("IP: {:?}", self.ip);
        println!("DP: {:?}", self.dp);
        // print non empty cell
        for (id, c) in self.cells.iter().enumerate() {
            if *c != 0 {
                println!("cell[{:?}] = {:?}", id, *c);
            }
        }
    }

    pub fn run(&mut self, debug: bool) -> Result<String> {
        let mut output = String::default();

        loop {
            // The program terminates when the instruction pointer
            // moves past the last command.
            if self.ip > self.insns.len() - 1 {
                break;
            }

            if debug {
                // As interpreter_state is using self.ip run it after
                // checking boundaries ^^^
                self.interpreter_state()
            }

            match self.insns[self.ip] {
                Token::Incptr => {
                    self.dp += 1;
                    if self.dp >= self.cells.len() {
                        eprintln!("Memory overflow");
                        return Err(());
                    }
                }
                Token::Decptr => {
                    if self.dp == 0 {
                        eprintln!("Memory underflow");
                        return Err(());
                    }
                    self.dp -= 1;
                }
                Token::Incbyte => self.cells[self.dp] += 1,
                Token::Decbyte => self.cells[self.dp] -= 1,
                Token::Outbyte => {
                    if let Some(c) = char::from_u32(self.cells[self.dp] as u32) {
                        output.push(c);
                    }
                }
                Token::Inbyte => {
                    let mut buf: [u8; 1] = [0];
                    match std::io::stdin().read(&mut buf) {
                        Err(e) => {
                            eprintln!("Failed to read byte from stdin: {}", e);
                            return Err(());
                        }
                        Ok(n) => {
                            if n > 0 {
                                self.cells[self.dp] = buf[0] as isize
                            }
                            // otherwise we are done
                        }
                    }
                }
                Token::Forward => {
                    if self.cells[self.dp] == 0 {
                        match self.jumps.get(&self.ip) {
                            Some(new_ip) => self.ip = *new_ip, // IP is incremented at the end
                            None => {
                                eprintln!("Failed to match bracket");
                                return Err(());
                            }
                        }
                    }
                }
                Token::Backward => {
                    if self.cells[self.dp] != 0 {
                        match self.jumps.get(&self.ip) {
                            Some(new_ip) => self.ip = *new_ip, // IP is incremented at the end
                            None => {
                                eprintln!("Failed to match bracket");
                                return Err(());
                            }
                        }
                    }
                }
            }

            self.ip += 1;
        }

        println!();
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::brainfuck::Interpreter;

    #[test]
    pub fn github_profile() {
        let mut prog = Interpreter::new(
            "
            +++++ +++ [ >+++++ + [ >+>++>++<<<- ] >>+>++>+ [ < ] <- ] >>>-.>++++.<+.>++.--------.
            ",
        )
        .unwrap();
        let output = prog.run(false).unwrap_or("FAILED".to_string());
        assert_eq!(output, "gthvn");
    }

    #[test]
    pub fn hello_test() {
        let mut prog = Interpreter::new(
            " source: wikipedia/Brainfuck
++++++++                Set Cell #0 to 8
[
    >++++               Add 4 to Cell #1; this will always set Cell #1 to 4
    [                   as the cell will be cleared by the loop
        >++             Add 2 to Cell #2
        >+++            Add 3 to Cell #3
        >+++            Add 3 to Cell #4
        >+              Add 1 to Cell #5
        <<<<-           Decrement the loop counter in Cell #1
    ]                   Loop until Cell #1 is zero; number of iterations is 4
    >+                  Add 1 to Cell #2
    >+                  Add 1 to Cell #3
    >-                  Subtract 1 from Cell #4
    >>+                 Add 1 to Cell #6
    [<]                 Move back to the first zero cell you find; this will
                        be Cell #1 which was cleared by the previous loop
    <-                  Decrement the loop Counter in Cell #0
]                       Loop until Cell #0 is zero; number of iterations is 8

The result of this is:
Cell no :   0   1   2   3   4   5   6
Contents:   0   0  72 104  88  32   8
Pointer :   ^

>>.                     Cell #2 has value 72 which is 'H'
>---.                   Subtract 3 from Cell #3 to get 101 which is 'e'
+++++++..+++.           Likewise for 'llo' from Cell #3
>>.                     Cell #5 is 32 for the space
<-.                     Subtract 1 from Cell #4 for 87 to give a 'W'
<.                      Cell #3 was set to 'o' from the end of 'Hello'
+++.------.--------.    Cell #3 for 'rl' and 'd'
>>+.                    Add 1 to Cell #5 gives us an exclamation point
>++.                    And finally a newline from Cell #6
            ",
        )
        .unwrap();
        let output = prog.run(false).unwrap_or("FAILED".to_string());
        assert_eq!(output, "Hello World!\n");
    }
}
