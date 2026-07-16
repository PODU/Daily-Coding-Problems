// Square root via Newton's method. ~O(log(1/eps)) iterations, O(1) space.
fn mysqrt(n: f64) -> f64 {
    if n == 0.0 {
        return 0.0;
    }
    let mut x = n;
    for _ in 0..200 {
        let nx = 0.5 * (x + n / x);
        if (nx - x).abs() < 1e-15 {
            x = nx;
            break;
        }
        x = nx;
    }
    x
}

fn main() {
    let n = 9.0;
    let r = mysqrt(n);
    if (r - r.round()).abs() < 1e-9 {
        println!("{}", r.round() as i64);
    } else {
        println!("{}", r);
    }
}
