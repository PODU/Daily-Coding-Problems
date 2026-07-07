// Min squared integers summing to n via Lagrange four-square + Legendre three-square theorem.
// O(sqrt(n)) per query (only the i^2+j^2 check loops), O(1) space.
fn is_perfect_square(n: i64) -> bool {
    let mut r = (n as f64).sqrt() as i64;
    while r * r < n {
        r += 1;
    }
    while r * r > n {
        r -= 1;
    }
    r * r == n
}

fn num_squares(n: i64) -> i32 {
    if is_perfect_square(n) {
        return 1;
    }
    let mut m = n;
    while m % 4 == 0 {
        m /= 4;
    }
    if m % 8 == 7 {
        return 4;
    }
    let mut i = 1i64;
    while i * i <= n {
        if is_perfect_square(n - i * i) {
            return 2;
        }
        i += 1;
    }
    3
}

fn main() {
    println!("{}", num_squares(13)); // 2
    println!("{}", num_squares(27)); // 3
}
