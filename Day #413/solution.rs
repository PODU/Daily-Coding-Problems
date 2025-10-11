// Day 413: Ordered ways to climb a staircase with allowed step sizes X.
// DP: ways[n] = sum over x in X of ways[n-x]. Time O(N*|X|), Space O(N).
fn count_ways(n: usize, steps: &[usize]) -> u64 {
    let mut ways = vec![0u64; n + 1];
    ways[0] = 1;
    for i in 1..=n {
        for &x in steps {
            if x <= i {
                ways[i] += ways[i - x];
            }
        }
    }
    ways[n]
}

fn main() {
    println!("{}", count_ways(4, &[1, 2]));      // 5
    println!("{}", count_ways(10, &[1, 3, 5]));  // generalized X={1,3,5}
}
