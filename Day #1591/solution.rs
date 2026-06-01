// Subset Sum: boolean DP over reachable sums; reconstruct one subset by backtracking.
// Time O(n*k), Space O(n*k).
fn subset_sum(s_arr: &[i32], k: usize) -> Option<Vec<i32>> {
    let n = s_arr.len();
    let mut reach = vec![vec![false; k + 1]; n + 1];
    reach[0][0] = true;
    for i in 1..=n {
        let v = s_arr[i - 1] as usize;
        for s in 0..=k {
            if reach[i - 1][s] {
                reach[i][s] = true;
            }
            if s >= v && reach[i - 1][s - v] {
                reach[i][s] = true;
            }
        }
    }
    if !reach[n][k] {
        return None;
    }
    let mut chosen = Vec::new();
    let mut s = k;
    for i in (1..=n).rev() {
        let v = s_arr[i - 1] as usize;
        if s >= v && reach[i - 1][s - v] {
            chosen.push(s_arr[i - 1]);
            s -= v;
        }
    }
    Some(chosen)
}

fn main() {
    let s_arr = [12, 1, 61, 5, 9, 2];
    let k = 24;
    let sub = subset_sum(&s_arr, k).unwrap_or_default();
    let parts: Vec<String> = sub.iter().map(|x| x.to_string()).collect();
    println!("[{}]", parts.join(", "));
    let total: i32 = sub.iter().sum();
    println!("Sum = {}", total);
}
