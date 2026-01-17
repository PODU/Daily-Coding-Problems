// Reflected binary Gray code: value i -> i ^ (i>>1) for i in 0..2^n. O(2^n) time/space.
fn gray_code(n: u32) -> Vec<String> {
    let total = 1u32 << n;
    (0..total)
        .map(|i| {
            let g = i ^ (i >> 1);
            (0..n)
                .rev()
                .map(|b| if (g >> b) & 1 == 1 { '1' } else { '0' })
                .collect::<String>()
        })
        .collect()
}

fn main() {
    let n = 2;
    println!("[{}]", gray_code(n).join(", "));
}
