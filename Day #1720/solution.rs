// Min perfect squares: Legendre/Lagrange three-square theorem gives the count in
// O(sqrt N); decomposition found greedily largest-square-first. Time O(sqrt N), Space O(1).

fn is_square(n: i64) -> bool {
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r * r == n
}

fn min_squares_count(n: i64) -> i32 {
    if is_square(n) {
        return 1;
    }
    let mut a = 1i64;
    while a * a <= n {
        if is_square(n - a * a) {
            return 2;
        }
        a += 1;
    }
    let mut m = n;
    while m % 4 == 0 {
        m /= 4;
    }
    if m % 8 == 7 {
        return 4;
    }
    3
}

fn find(n: i64, count: i32, out: &mut Vec<i64>) -> bool {
    if count == 1 {
        if is_square(n) {
            out.push(n);
            return true;
        }
        return false;
    }
    let start = (n as f64).sqrt() as i64 + 1;
    for a in (1..=start).rev() {
        if a * a > n {
            continue;
        }
        if find(n - a * a, count - 1, out) {
            out.push(a * a);
            return true;
        }
    }
    false
}

fn demo(n: i64) {
    let count = min_squares_count(n);
    let mut parts: Vec<i64> = Vec::new();
    find(n, count, &mut parts);
    parts.sort_unstable_by(|a, b| b.cmp(a));
    let strs: Vec<String> = parts.iter().map(|p| p.to_string()).collect();
    println!("{} ({})", count, strs.join(" + "));
}

fn main() {
    demo(4);
    demo(17);
    demo(18);
}
