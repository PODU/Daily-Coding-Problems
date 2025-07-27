// Single number where others appear 3x: ones/twos bit-counting state machine.
// O(N) time, O(1) space. Works for negatives via two's-complement i32.
fn single_number(nums: &[i32]) -> i32 {
    let mut ones = 0;
    let mut twos = 0;
    for &x in nums {
        ones = (ones ^ x) & !twos;
        twos = (twos ^ x) & !ones;
    }
    ones
}

fn main() {
    println!("{}", single_number(&[6, 1, 3, 3, 3, 6, 6]));
}
