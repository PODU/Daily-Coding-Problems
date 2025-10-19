// Day 459: Fewest perfect squares summing to N.
// Lagrange/Legendre theorem -> answer in {1,2,3,4}, O(sqrt N).
// Reconstruct one decomposition guided by the count.
fn is_square(n: i64) -> bool {
    if n < 0 {
        return false;
    }
    let mut r = (n as f64).sqrt() as i64;
    while r * r < n {
        r += 1;
    }
    while r * r > n {
        r -= 1;
    }
    r * r == n
}

fn min_squares(n: i64) -> i32 {
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
    let mut i = 1i64;
    while i * i <= n {
        if is_square(n - i * i) {
            return 2;
        }
        i += 1;
    }
    3
}

fn decompose(mut n: i64) -> Vec<i64> {
    let mut k = min_squares(n);
    let mut res = Vec::new();
    while k > 0 {
        if k == 1 {
            res.push(n);
            break;
        }
        let mut i = (n as f64).sqrt() as i64;
        while i >= 1 {
            if min_squares(n - i * i) == k - 1 {
                res.push(i * i);
                n -= i * i;
                k -= 1;
                break;
            }
            i -= 1;
        }
    }
    res
}

fn main() {
    let n = 17;
    let sq = decompose(n);
    let parts: Vec<String> = sq.iter().map(|x| x.to_string()).collect();
    println!("{} ({})", min_squares(n), parts.join(" + ")); // 2 (16 + 1)
}
