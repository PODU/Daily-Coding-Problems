// Gray code via reflect-and-prefix formula g(i) = i XOR (i>>1).
// Time: O(n * 2^n) to format. Space: O(2^n).
fn gray_code(n: u32) -> Vec<String> {
    (0..(1u32 << n))
        .map(|i| {
            let g = i ^ (i >> 1);
            format!("{:0width$b}", g, width = n as usize)
        })
        .collect()
}

fn main() {
    println!("[{}]", gray_code(2).join(", ")); // [00, 01, 11, 10]
}
