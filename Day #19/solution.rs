// Paint House: DP tracking min cost per color using min1/min2 trick.
// Time O(N*K), Space O(1) extra.
fn min_cost(cost: &Vec<Vec<i32>>) -> i32 {
    if cost.is_empty() {
        return 0;
    }
    let k = cost[0].len();
    let mut prev = cost[0].clone();
    for i in 1..cost.len() {
        let (mut min1, mut min2): (i32, i32) = (-1, -1);
        for j in 0..k {
            if min1 == -1 || prev[j] < prev[min1 as usize] {
                min2 = min1;
                min1 = j as i32;
            } else if min2 == -1 || prev[j] < prev[min2 as usize] {
                min2 = j as i32;
            }
        }
        let mut cur = vec![0; k];
        for j in 0..k {
            let best = if j as i32 == min1 { prev[min2 as usize] } else { prev[min1 as usize] };
            cur[j] = cost[i][j] + best;
        }
        prev = cur;
    }
    *prev.iter().min().unwrap()
}

fn main() {
    let cost = vec![vec![1, 5, 3], vec![2, 9, 4]];
    println!("Minimum cost = {}", min_cost(&cost));
}
