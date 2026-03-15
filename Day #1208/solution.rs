// Day 1208: Fewest perfect squares summing to N.
// Lagrange four-square + Legendre's three-square theorem. Time O(sqrt N), Space O(1).
fn is_square(n: u64) -> bool {
    let r = (n as f64).sqrt() as u64;
    (r.saturating_sub(1)..=r + 1).any(|x| x * x == n)
}

fn num_squares(n: u64) -> u32 {
    if is_square(n) {
        return 1;
    }
    let mut a = 1u64;
    while a * a <= n {
        if is_square(n - a * a) {
            return 2;
        }
        a += 1;
    }
    let mut m = n;
    while m % 4 == 0 {
        m /= 4;
    }
    if m % 8 == 7 {
        4
    } else {
        3
    }
}

fn main() {
    println!("{}", num_squares(17)); // 2 (16 + 1)
}
