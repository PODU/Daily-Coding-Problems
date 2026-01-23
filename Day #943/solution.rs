// Day 943: Count tilings of a 2xN board with 2x1 dominoes and L-trominoes.
// DP recurrence f(n) = 2*f(n-1) + f(n-3), f(0)=1,f(1)=1,f(2)=2. Time O(n), Space O(1).
fn count_tilings(n: usize) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        _ => {
            let (mut a, mut b, mut c) = (1u64, 1u64, 2u64); // f(0), f(1), f(2)
            for _ in 3..=n {
                let f = 2 * c + a;
                a = b;
                b = c;
                c = f;
            }
            c
        }
    }
}

fn main() {
    println!("{}", count_tilings(4)); // 11
}
