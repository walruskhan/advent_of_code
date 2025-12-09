const std = @import("std");
const ArrayList = std.ArrayList;
const test_allocator = std.testing.allocator;
const Reader = std.Io.Reader;
const File = std.fs.File;
const Allocator = std.mem.Allocator;

fn read_file(allocator: Allocator, path: []const u8) ![]u8 {
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();

    const length = try file.getEndPos();

    const buffer = try std.mem.Allocator.alloc(allocator, u8, length);
    var reader = file.reader(buffer);
    try reader.interface.fill(length);

    return buffer;
}

pub fn part1() !void {
    const allocator = std.heap.page_allocator;

    const raw_text = try read_file(allocator, "input2.txt");

    var position: i32 = 50;
    var amount_zero: i32 = 0;

    var lines = std.mem.splitScalar(u8, raw_text, '\n');
    while (lines.next()) |line| {
        const dir: u8 = line[0];
        const raw_amt: i32 = try std.fmt.parseInt(i32, line[1..], 10);

        const delta: i32 = @intCast(@abs(raw_amt) % 100);

        if (dir == 'L' or dir == 'l') {
            position -= delta;
        } else {
            position += delta;
        }

        if (position < 0) {
            position += 100;
        } else if (position >= 100) {
            position -= 100;
        }

        if (position == 0) {
            amount_zero += 1;
        }

        std.debug.print("Rotated {c} by {d} ({d}) to point at {d}\n", .{ dir, raw_amt, delta, position });
    }

    std.debug.print("Final position = {d} code = {d}", .{ position, amount_zero });
}

pub fn part2() !void {
    const allocator = std.heap.page_allocator;

    const raw_text = try read_file(allocator, "input2.txt");

    var position: i32 = 50;
    var amount_zero: i32 = 0;

    var lines = std.mem.splitScalar(u8, raw_text, '\n');
    while (lines.next()) |line| {
        // std.debug.print("{s}", .{line});

        const dir: u8 = line[0];
        const amt: i32 = try std.fmt.parseInt(i32, line[1..], 10);

        if (dir == 'L' or dir == 'l') {
            position -= amt;
        } else {
            position += amt;
        }

        const v = @abs(amt) / 100;
        amount_zero += @intCast(v);

        while (position < 0) {
            position += 100;
            amount_zero += 1;
        }

        while (position >= 100) {
            position -= 100;
            amount_zero += 1;
        }

        if (position == 0) {
            amount_zero += 1;
        }

        std.debug.print("Rotated {c} by {d} to point at {d}\n", .{ dir, amt, position });
    }

    std.debug.print("Final position = {d} code = {d}", .{ position, amount_zero });
}

pub fn main() !void {
    try part1();
}

test "simple test" {
    const gpa = std.testing.allocator;
    var list: std.ArrayList(i32) = .empty;
    defer list.deinit(gpa); // Try commenting this out and see if zig detects the memory leak!
    try list.append(gpa, 42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}

test "fuzz example" {
    const Context = struct {
        fn testOne(context: @This(), input: []const u8) anyerror!void {
            _ = context;
            // Try passing `--fuzz` to `zig build test` and see if it manages to fail this test case!
            try std.testing.expect(!std.mem.eql(u8, "canyoufindme", input));
        }
    };
    try std.testing.fuzz(Context{}, Context.testOne, .{});
}
