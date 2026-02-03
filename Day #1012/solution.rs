// Square root via Newton's method: x = (x + n/x)/2, quadratic convergence.
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
        if (nx - x).abs() < 1e-15 {
            x = nx;
            break;
        }
        x = nx;
    }
    x
}

fn print_result(n: f64) {
    let r = my_sqrt(n);
    let ri = r.round();
    if (r - ri).abs() < 1e-9 {
        println!("{}", ri as i64); // exact integer
    } else {
        println!("{:.8}", r);
    }
}

fn main() {
    print_result(9.0); // -> 3
    print_result(2.0); // -> 1.41421356
}
