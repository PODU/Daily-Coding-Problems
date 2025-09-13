// Day 268: Power of four check in O(1).
// Power of two (n & (n-1))==0 AND single bit at even position (n & 0x55555555). Time O(1), Space O(1).

fn is_power_of_four(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0 && (n & 0x5555_5555) != 0
}

fn main() {
    for t in [16u32, 8, 64, 5] {
        let res = if is_power_of_four(t) { "True" } else { "False" };
        println!("{} -> {}", t, res);
    }
}
