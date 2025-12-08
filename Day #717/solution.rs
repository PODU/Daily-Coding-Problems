// Day 717: Min cost to paint N houses, K colors, no two adjacent same color.
// DP over rows tracking best & second-best previous costs. Time O(N*K), space O(1).
fn min_cost(costs: &[Vec<i32>]) -> i32 {
    if costs.is_empty() {
        return 0;
    }
    let k = costs[0].len();
    let mut prev = costs[0].clone();
    for row in costs.iter().skip(1) {
        let (mut m1, mut m2, mut idx) = (i32::MAX, i32::MAX, usize::MAX);
        for j in 0..k {
            if prev[j] < m1 {
                m2 = m1;
                m1 = prev[j];
                idx = j;
            } else if prev[j] < m2 {
                m2 = prev[j];
            }
        }
        let mut cur = vec![0; k];
        for j in 0..k {
            cur[j] = row[j] + if j == idx { m2 } else { m1 };
        }
        prev = cur;
    }
    *prev.iter().min().unwrap()
}

fn main() {
    let costs = vec![vec![1, 5, 3], vec![2, 9, 4]];
    println!("{}", min_cost(&costs));
}
