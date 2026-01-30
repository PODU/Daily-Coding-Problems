// Day 990: Count ordered ways to climb N steps using step sizes from set X.
// Bottom-up DP: ways[i] = sum over x in X of ways[i-x]. O(N*|X|) time, O(N) space.

fn climb_ways(n: usize, x_set: &[usize]) -> u64 {
    let mut ways = vec![0u64; n + 1];
    ways[0] = 1;
    for i in 1..=n {
        for &x in x_set {
            if i >= x {
                ways[i] += ways[i - x];
            }
        }
    }
    ways[n]
}

fn main() {
    println!("N=4, X={{1,2}}: {}", climb_ways(4, &[1, 2]));       // expected 5
    println!("N=4, X={{1,3,5}}: {}", climb_ways(4, &[1, 3, 5]));  // generalized
}
