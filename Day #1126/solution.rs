// Smallest number of perfect squares summing to N.
// Legendre/Lagrange: 1 if perfect square; 4 if N=4^a(8b+7); 2 if sum of two squares; else 3. O(sqrt(N)) time, O(1) space.
fn is_square(n: i64) -> bool {
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
    let n: i64 = 17;
    println!("{}", num_squares(n));
}
