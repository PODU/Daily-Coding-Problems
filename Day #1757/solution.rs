// Day 1757: Count ordered ways to climb N stairs (steps 1 or 2 -> Fibonacci).
// Generalized to a step set X via DP: ways[i] = sum of ways[i-x]. O(N*|X|) time, O(N) space.

fn climb_ways(n: usize, steps: &[usize]) -> u64 {
    let mut ways = vec![0u64; n + 1];
    ways[0] = 1;
    for i in 1..=n {
        for &x in steps {
            if i >= x {
                ways[i] += ways[i - x];
            }
        }
    }
    ways[n]
}

fn main() {
    let n = 4;
    println!("{}", climb_ways(n, &[1, 2])); // 5
    println!("Generalized X={{1,3,5}}, N=4: {}", climb_ways(n, &[1, 3, 5]));
}
