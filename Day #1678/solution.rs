// Day 1678: Integer division without / * %. Subtract largest shifted multiple of
// divisor each round (doubling). Time O(log(quotient)), Space O(1).
fn divide(mut a: u64, b: u64) -> u64 {
    let mut q = 0u64;
    while a >= b {
        let (mut temp, mut mult) = (b, 1u64);
        while a >= temp << 1 {
            temp <<= 1;
            mult <<= 1;
        }
        a -= temp;
        q += mult;
    }
    q
}

fn main() {
    println!("{}", divide(43, 8)); // 5
}
