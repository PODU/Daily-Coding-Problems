// DP over rows, tracking the two smallest running totals so each house picks
// the best previous color != its own. Time O(N*K), Space O(1) extra.

fn min_cost(costs: &[Vec<i32>]) -> i32 {
    if costs.is_empty() {
        return 0;
    }
    let k = costs[0].len();
    let mut prev = costs[0].clone();
    for row in costs.iter().skip(1) {
        let (mut m1, mut m2, mut idx1) = (i32::MAX, i32::MAX, usize::MAX);
        for j in 0..k {
            if prev[j] < m1 {
                m2 = m1;
                m1 = prev[j];
                idx1 = j;
            } else if prev[j] < m2 {
                m2 = prev[j];
            }
        }
        let cur: Vec<i32> = (0..k)
            .map(|j| row[j] + if j == idx1 { m2 } else { m1 })
            .collect();
        prev = cur;
    }
    *prev.iter().min().unwrap()
}

fn main() {
    let costs = vec![vec![1, 5, 3], vec![2, 9, 4]];
    println!("{}", min_cost(&costs)); // 5
}
