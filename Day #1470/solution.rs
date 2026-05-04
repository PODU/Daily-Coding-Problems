// Least squares summing to n via Legendre/Lagrange: 1 if square, 4 if 4^a(8b+7),
// 2 if i^2+j^2, else 3. Time: O(sqrt(n)); Space: O(1).
fn is_square(x: i64) -> bool {
    let mut r = (x as f64).sqrt() as i64;
    while r * r < x {
        r += 1;
    }
    while r * r > x {
        r -= 1;
    }
    r * r == x
}

fn num_squares(n: i64) -> i32 {
    if is_square(n) {
        return 1;
    }
    let mut m = n;
    while m % 4 == 0 {
        m /= 4;
    }
    if m % 8 == 7 {
        return 4;
    }
    let mut i: i64 = 1;
    while i * i <= n {
        if is_square(n - i * i) {
            return 2;
        }
        i += 1;
    }
    3
}

fn main() {
    println!("{}", num_squares(13));
    println!("{}", num_squares(27));
}
