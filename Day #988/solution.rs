// Day 988: Minimum number of perfect squares summing to n.
// Number theory (Lagrange + Legendre's three-square theorem). O(sqrt n) time, O(1) space.

fn is_square(x: i64) -> bool {
    let mut r = (x as f64).sqrt() as i64;
    while r * r > x { r -= 1; }
    while (r + 1) * (r + 1) <= x { r += 1; }
    r * r == x
}

fn num_squares(n: i64) -> i32 {
    if is_square(n) {
        return 1;
    }
    let mut m = n;
    while m % 4 == 0 { // strip factors of 4
        m /= 4;
    }
    if m % 8 == 7 { // Legendre: 4^a(8b+7) needs 4
        return 4;
    }
    let mut a = 1i64;
    while a * a <= n {
        if is_square(n - a * a) {
            return 2;
        }
        a += 1;
    }
    3
}

fn main() {
    println!("13 -> {}", num_squares(13)); // expected 2
    println!("27 -> {}", num_squares(27)); // expected 3
}
