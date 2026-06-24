// Power of four: N>0 && single set bit (N&(N-1))==0 && bit at even position (N & 0x55555555). O(1).
fn is_power_of_four(n: u32) -> bool {
    n > 0 && (n & (n - 1)) == 0 && (n & 0x5555_5555) != 0
}

fn main() {
    for t in [16u32, 8, 1, 64, 5] {
        println!("{} -> {}", t, is_power_of_four(t));
    }
}
