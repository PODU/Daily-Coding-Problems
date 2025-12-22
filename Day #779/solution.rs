// Day 779: Egg drop - minimum worst-case trials for N eggs, k floors.
// Find smallest m with sum_{i=1..N} C(m,i) >= k. O(N * answer) time, O(1) space.
fn egg_drop(eggs: i64, floors: i64) -> i64 {
    if floors == 0 {
        return 0;
    }
    let mut m = 0i64;
    loop {
        m += 1;
        let mut reach: i64 = 0;
        let mut c: i64 = 1;
        for i in 1..=eggs {
            c = c * (m - i + 1) / i;
            reach += c;
            if reach >= floors {
                break;
            }
        }
        if reach >= floors {
            return m;
        }
    }
}

fn main() {
    println!("{}", egg_drop(1, 5));   // 5
    println!("{}", egg_drop(2, 100)); // 14
}
