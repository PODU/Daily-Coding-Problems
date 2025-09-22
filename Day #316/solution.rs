// Reconstruct coin denominations from a ways-to-make-change array.
// DP coin detection: A[i] > ways[i] means i is a coin. Time O(N^2), Space O(N).

fn join_coins(c: &[usize]) -> String {
    match c.len() {
        0 => String::new(),
        1 => c[0].to_string(),
        2 => format!("{} and {}", c[0], c[1]),
        _ => {
            let head: Vec<String> = c[..c.len() - 1].iter().map(|x| x.to_string()).collect();
            format!("{}, and {}", head.join(", "), c[c.len() - 1])
        }
    }
}

fn find_coins(a: &[i64]) -> Vec<usize> {
    let n = a.len();
    let mut ways = vec![0i64; n];
    ways[0] = 1;
    let mut coins = Vec::new();
    for i in 1..n {
        if a[i] > ways[i] {
            coins.push(i);
            for j in i..n {
                ways[j] += ways[j - i];
            }
        }
    }
    coins
}

fn main() {
    let a: Vec<i64> = vec![1, 0, 1, 1, 2];
    println!("{}", join_coins(&find_coins(&a)));
}
