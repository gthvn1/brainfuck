const data = @embedFile("./data.bf");
const brainfuck = @import("./brainfuck.zig").BrainFuck();
const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    print("> Running brainfuck\n", .{});

    var bf = brainfuck{};
    try bf.parse(data);
    try bf.execute();

    print("\n> Done.\n", .{});
}
