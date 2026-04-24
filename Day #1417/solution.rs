// Day 1417: count tilings of a 2xN board with dominoes and L-trominoes.
// Approach: DP recurrence f(n) = 2*f(n-1) + f(n-3), f(0)=1,f(1)=1,f(2)=2. O(n) time, O(1) space.

fn count_tilings(n: u32) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        _ => {
            let (mut a, mut b, mut c): (u64, u64, u64) = (1, 1, 2);
            for _ in 3..=n {
                let cur = 2 * c + a;
                a = b;
                b = c;
                c = cur;
            }
            c
        }
    }
}

fn main() {
    println!("{}", count_tilings(4)); // 11
}
