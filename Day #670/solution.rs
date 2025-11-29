// Day 670: Fewest perfect squares summing to n. Lagrange (answer in {1,2,3,4}) +
// Legendre three-square theorem. Time O(sqrt n), Space O(1).
fn is_square(n: i64) -> bool {
    let r = (n as f64).sqrt() as i64;
    (r * r == n) || ((r + 1) * (r + 1) == n)
}

fn num_squares(n: i64) -> i32 {
    if is_square(n) {
        return 1;
    }
    let mut a = 1i64;
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
    println!("{}", num_squares(13)); // 2
    println!("{}", num_squares(27)); // 3
}
