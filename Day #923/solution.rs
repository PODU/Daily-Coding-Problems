// Power of four via O(1) bit manipulation.
// power of two: n>0 && (n&(n-1))==0; set bit in even position: (n & 0x55555555)!=0.
// Time O(1), Space O(1).
fn is_power_of_four(n: u32) -> bool {
    n > 0 && (n & (n - 1)) == 0 && (n & 0x5555_5555) != 0
}

fn main() {
    let tests = [1u32, 4, 16, 64, 8, 5, 0];
    for n in tests.iter() {
        println!("{}: {}", n, is_power_of_four(*n));
    }
}
