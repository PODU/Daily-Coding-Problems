// Day 718: Gray code for n bits via formula g(i) = i ^ (i >> 1). Produces 2^n
// values where consecutive (and wrap-around) differ by one bit. Time O(2^n).
fn gray_code(n: u32) -> Vec<String> {
    (0..(1u32 << n))
        .map(|i| {
            let g = i ^ (i >> 1);
            (0..n).rev().map(|b| if (g >> b) & 1 == 1 { '1' } else { '0' }).collect()
        })
        .collect()
}

fn main() {
    let codes = gray_code(2);
    println!("[{}]", codes.join(", "));
}
