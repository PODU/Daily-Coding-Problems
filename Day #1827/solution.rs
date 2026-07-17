// Egg drop: min trials t s.t. floors(t,eggs)=sum_{i=1..eggs} C(t,i) >= k.
// O(eggs * answer). For N=1,k=5 -> 5.
fn floors_covered(t: i64, eggs: i64, cap: i64) -> i64 {
    let mut total: i64 = 0;
    let mut c: i64 = 1;
    for i in 1..=eggs {
        c = c * (t - i + 1) / i; // C(t,i) incrementally
        total += c;
        if total >= cap {
            return cap;
        }
    }
    total
}

fn min_drops(eggs: i64, k: i64) -> i64 {
    let mut t = 0;
    while floors_covered(t, eggs, k) < k {
        t += 1;
    }
    t
}

fn main() {
    println!("{}", min_drops(1, 5)); // 5
}
