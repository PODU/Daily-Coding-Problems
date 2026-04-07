// Square root of a real number via Newton's method: x <- (x + n/x)/2.
// Quadratic convergence -> O(log(1/eps)) iterations, O(1) space.

fn my_sqrt(n: f64) -> f64 {
    assert!(n >= 0.0, "negative");
    if n == 0.0 {
        return 0.0;
    }
    let mut x = n;
    for _ in 0..100 {
        let nx = 0.5 * (x + n / x);
        if (nx - x).abs() < 1e-15 {
            break;
        }
        x = nx;
    }
    x
}

fn main() {
    let r = my_sqrt(9.0);
    if (r - r.round()).abs() < 1e-9 {
        println!("{}", r.round() as i64); // 3
    } else {
        println!("{}", r);
    }
}
