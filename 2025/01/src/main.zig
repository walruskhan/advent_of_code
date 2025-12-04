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

pub fn main() !void {
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

        while (position < 0) {
            position += 100;
        }

        while (position >= 100) {
            position -= 100;
        }

        if (position == 0) {
            amount_zero += 1;
        }

        std.debug.print("Rotated {c} by {d} to point at {d}\n", .{ dir, amt, position });
    }

    std.debug.print("Final position = {d} code = {d}", .{ position, amount_zero });

    // var line_iter = std.mem.splitScalar(u8, test_data, '\n');

    // var r: Reader = .fixed("ab\ncd");
    // const result = r.takeDelimiterInclusive('\n');
    // r.tossBuffered();
    // std.debug.print("{any}", .{result});

    // const result2 = r.takeDelimiterInclusive('\n');
    // _ = try r.take(1);
    // std.debug.print("{any}", .{result2});

    // const result3 = r.takeDelimiterInclusive('\n');
    // std.debug.print("{any}", .{result3});

    // const file = try std.fs.cwd().openFile("input1.txt", .{});
    // defer file.close();

    // var read_buffer: [1024]u8 = undefined;
    // var reader = file.reader(&read_buffer);

    // while (reader.interface.takeDelimiterExclusive('\n')) |line| {
    //     std.debug.print("{s}", .{line});
    // } else |err| switch (err) {
    //     error.EndOfStream => {},
    //     error.ReadFailed => return err,
    //     error.StreamTooLong => return err,
    // }

    // var stdout_buffer: [512]u8 = undefined;
    // const stdout: std.Io.Writer = stdout_writer.interface;

    // const file = try std.fs.cwd().openFile("input1.txt", .{ .mode = .read_only });
    // defer file.close();

    // var read_buffer: [1024]u8 = undefined;
    // var reader: std.io.Reader = file.reader(&read_buffer).interface;

    // while (reader.takeDelimiterExclusive('\n')) |line| {
    //     // `line` is a slice of bytes (excluding the delimiter)
    //     // do whatever you want with it

    //     try stdout.writeAll("You typed: ");
    //     try stdout.print("{s}", .{line});
    //     try stdout.writeAll("\n...\n");
    //     try stdout.writeAll("Type something: ");

    //     try stdout.flush();
    // } else |err| switch (err) {
    //     error.EndOfStream => {
    //         // reached end
    //         // the normal case
    //     },
    //     error.StreamTooLong => {
    //         // the line was longer than the internal buffer
    //         return err;
    //     },
    //     error.ReadFailed => {
    //         // the read failed
    //         return err;
    //     },
    // }
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
