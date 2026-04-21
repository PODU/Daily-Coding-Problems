// Power of four iff: positive, single set bit (n & (n-1))==0, and that bit sits
// at an even position (n & 0x55555555). O(1) time, O(1) space.

fn is_power_of_four(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0 && (n & 0x5555_5555) != 0
}

fn main() {
    for n in [1u32, 4, 16, 5, 64, 63] {
        println!("{} -> {}", n, is_power_of_four(n));
    }
}
