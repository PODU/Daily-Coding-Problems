// Gray code generation via reflect formula i ^ (i>>1). Time O(2^n), Space O(2^n).

fn gray_code(n: u32) -> Vec<String> {
    (0..(1u32 << n))
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
    let codes = gray_code(n);
    println!("[{}]", codes.join(", "));
}
