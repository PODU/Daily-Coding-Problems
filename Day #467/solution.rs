// Square root via Newton's method: x = (x + n/x)/2 until convergence.
// Time: O(log(1/eps)) iterations, Space: O(1).
fn my_sqrt(n: f64) -> f64 {
    if n < 0.0 {
        return f64::NAN;
    }
    if n == 0.0 {
        return 0.0;
    }
    let mut x = n;
    for _ in 0..100 {
        let nx = 0.5 * (x + n / x);
        if (nx - x).abs() < 1e-12 {
            break;
        }
        x = nx;
    }
    x
}

fn main() {
    let n = 9.0;
    let r = my_sqrt(n);
    if (r - r.round()).abs() < 1e-9 {
        println!("{}", r.round() as i64);
    } else {
        println!("{}", r);
    }
}
