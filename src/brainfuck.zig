//! Brainfuck interpreter
const std = @import("std");

pub fn BrainFuck() type {
    return struct {
        const Self = @This();
        const maxTokens: u16 = 4096;
        const maxCells: u16 = 32;

        const BrainFuckError = error{
            TokensOverflow,
            TokensUnderflow,
            CellsOverflow,
            CellsUnderflow,
            JumpsError,
        };

        const Token = enum(u8) {
            stop = 0, // use as sentinel
            incPtr,
            decPtr,
            incCell,
            decCell,
            output,
            input,
            openBracket,
            closeBracket,
        };

        idx: u16 = 0,
        nestedJumps: u16 = 0,
        tokens: [maxTokens]Token = [_]Token{Token.stop} ** maxTokens,
        cells: [maxCells]u32 = [_]u32{0} ** maxCells,

        pub fn parse(self: *Self, code: []const u8) !void {
            var idx: u16 = 0;
            for (code) |c| {
                switch (c) {
                    '>' => self.tokens[idx] = Token.incPtr,
                    '<' => self.tokens[idx] = Token.decPtr,
                    '+' => self.tokens[idx] = Token.incCell,
                    '-' => self.tokens[idx] = Token.decCell,
                    '.' => self.tokens[idx] = Token.output,
                    ',' => self.tokens[idx] = Token.input,
                    '[' => {
                        self.tokens[idx] = Token.openBracket;
                    },
                    ']' => self.tokens[idx] = Token.closeBracket,
                    else => continue,
                }

                idx += 1;
                if (idx == maxTokens)
                    return BrainFuckError.TokensOverflow;
            }
        }

        pub fn execute(self: *Self) !void {
            var tokenIdx: u16 = 0;
            var cellIdx: u16 = 0;

            while (true) {
                switch (self.tokens[tokenIdx]) {
                    Token.stop => break,
                    Token.incPtr => {
                        //std.debug.print("Executing token {any} @ {}\n ", .{ self.tokens[tokenIdx], tokenIdx });
                        // Increment data pointer by one (to point to the next cell to the right).
                        cellIdx += 1;
                        if (cellIdx == maxCells) return BrainFuckError.CellsOverflow;
                    },
                    Token.decPtr => {
                        //std.debug.print("Executing token {any} @ {}\n ", .{ self.tokens[tokenIdx], tokenIdx });
                        // Decrement data pointer by one (to point to the next cell to the left).
                        if (cellIdx == 0) return BrainFuckError.CellsUnderflow;
                        cellIdx -= 1;
                    },
                    Token.incCell => {
                        //std.debug.print("Executing token {any} @ {}\n ", .{ self.tokens[tokenIdx], tokenIdx });
                        // Increment the byte at the data pointer by one.
                        self.cells[cellIdx] += 1;
                    },
                    Token.decCell => {
                        //std.debug.print("Executing token {any} @ {}\n ", .{ self.tokens[tokenIdx], tokenIdx });
                        // Decrement the byte at the data pointer by one.
                        self.cells[cellIdx] -= 1;
                    },
                    Token.output => {
                        //std.debug.print("Executing token {any} @ {}\n ", .{ self.tokens[tokenIdx], tokenIdx });
                        // Output the byte at the data pointer.
                        if (self.cells[cellIdx] <= 255) {
                            const c: u8 = @truncate(self.cells[cellIdx]);
                            if (std.ascii.isPrint(c)) {
                                std.debug.print("{c}", .{c});
                            } else {
                                std.debug.print("<non printable u8 {}>", .{c});
                            }
                        } else {
                            std.debug.print("{}", .{self.cells[cellIdx]});
                        }
                    },
                    Token.input => {
                        //std.debug.print("Executing token {any} @ {}\n ", .{ self.tokens[tokenIdx], tokenIdx });
                        // Accept one byte of input, storing its value in the byte at the data pointer.
                        std.debug.print("Read byte not implemented\n", .{});
                    },
                    Token.openBracket => {
                        //std.debug.print("Executing token {any} @ {}\n ", .{ self.tokens[tokenIdx], tokenIdx });
                        // If the byte at the data pointer is zero, then instead of moving the instruction
                        // pointer forward to the next command, jump it forward to the command after the
                        // matching ] command.

                        // Check that nested jumps is equal to 0. Nested jump are managed here.
                        if (self.nestedJumps != 0)
                            return BrainFuckError.JumpsError;

                        if (self.cells[cellIdx] == 0) {
                            while (true) {
                                tokenIdx += 1;

                                if (tokenIdx == maxTokens)
                                    return BrainFuckError.JumpsError;

                                if (self.tokens[tokenIdx] == Token.closeBracket) {
                                    if (self.nestedJumps == 0) {
                                        break;
                                    } else {
                                        self.nestedJumps -= 1;
                                        continue;
                                    }
                                }

                                if (self.tokens[tokenIdx] == Token.openBracket)
                                    self.nestedJumps += 1;
                            }
                        } // else nothing to do
                    },
                    Token.closeBracket => {
                        //std.debug.print("Executing token {any} @ {}\n ", .{ self.tokens[tokenIdx], tokenIdx });
                        // If the byte at the data pointer is nonzero, then instead of moving the
                        // instruction pointer forward to the next command, jump it back to the command
                        // after the matching [ command.
                        if (self.nestedJumps != 0)
                            return BrainFuckError.JumpsError;

                        if (self.cells[cellIdx] != 0) {
                            while (true) {
                                tokenIdx -= 1;

                                if (tokenIdx == 0)
                                    return BrainFuckError.TokensUnderflow;

                                if (self.tokens[tokenIdx] == Token.openBracket) {
                                    if (self.nestedJumps == 0) {
                                        break;
                                    } else {
                                        self.nestedJumps -= 1;
                                        continue;
                                    }
                                }

                                if (self.tokens[tokenIdx] == Token.closeBracket)
                                    self.nestedJumps += 1;
                            }
                        } // else nothing to do
                    },
                }

                tokenIdx += 1;
                if (tokenIdx == maxTokens)
                    return BrainFuckError.TokensOverflow;
            }
        }
    };
}
