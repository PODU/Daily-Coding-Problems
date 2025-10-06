// Integer division without '/': repeated doubling/subtraction.
// Time: O(log n), Space: O(1).

fn divide(a: i64, b: i64) -> (i64, i64) {
    let neg = (a < 0) != (b < 0);
    let (mut x, y) = (a.abs(), b.abs());
    let mut q: i64 = 0;
    while x >= y {
        let (mut temp, mut mult) = (y, 1i64);
        while x >= (temp << 1) {
            temp <<= 1;
            mult <<= 1;
        }
        x -= temp;
        q += mult;
    }
    (if neg { -q } else { q }, x)
}

fn main() {
    let (q, r) = divide(10, 3);
    println!("({}, {})", q, r);
}
