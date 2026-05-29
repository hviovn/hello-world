d:\archive\github\Github_hviovn\hello-world\examples\hello.zigconst std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    
    // The 1,000,000th prime is 15,485,863. 
    // We add a small buffer to ensure we capture it.
    const limit = 15_500_000;
    const target_count = 1_000_000;

    // Get an allocator for heap memory
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Allocate the sieve array (true means potentially prime)
    const sieve = try allocator.alloc(bool, limit);
    defer allocator.free(sieve);
    
    // Initialize all elements to true
    @memset(sieve, true);

    var prime_count: usize = 0;
    var last_prime: usize = 0;

    // Sieve of Eratosthenes
    for (2..limit) |p| {
        if (sieve[p]) {
            prime_count += 1;
            last_prime = p;

            if (prime_count == target_count) break;

            // Mark multiples as composite
            var i = p * p;
            while (i < limit) : (i += p) {
                sieve[i] = false;
            }
        }
    }

    try stdout.print("Successfully found {d} primes.\n", .{prime_count});
    try stdout.print("The 1,000,000th prime is: {d}\n", .{last_prime});
}
