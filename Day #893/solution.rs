// Paint houses: DP tracking two smallest costs of previous row -> O(N*K) time, O(1) extra space.
// For each house we only need the min and second-min of the previous row to avoid same color.

fn min_cost(costs: &[Vec<i32>]) -> i32 {
    if costs.is_empty() {
        return 0;
    }
    let (mut prev_min, mut prev_second, mut prev_idx) = (0, 0, -1i32);
    for row in costs {
        let (mut cur_min, mut cur_second, mut cur_idx) = (i32::MAX, i32::MAX, -1i32);
        for (c, &base) in row.iter().enumerate() {
            let cost = base + if c as i32 == prev_idx { prev_second } else { prev_min };
            if cost < cur_min {
                cur_second = cur_min;
                cur_min = cost;
                cur_idx = c as i32;
            } else if cost < cur_second {
                cur_second = cost;
            }
        }
        prev_min = cur_min;
        prev_second = cur_second;
        prev_idx = cur_idx;
    }
    prev_min
}

fn main() {
    let costs = vec![vec![1, 5, 3], vec![2, 9, 4]];
    println!("{}", min_cost(&costs));
}
