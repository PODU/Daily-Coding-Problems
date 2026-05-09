// Day 1496: Min cost to paint N houses with K colors, no two adjacent same color.
// Approach: DP tracking previous row's min & 2nd-min (+min index). Time O(N*K), Space O(1).

fn min_cost(costs: &[Vec<i64>]) -> i64 {
    if costs.is_empty() {
        return 0;
    }
    let (mut prev_min1, mut prev_min2, mut prev_idx): (i64, i64, i32) = (0, 0, -1);
    for row in costs {
        let (mut cur_min1, mut cur_min2, mut cur_idx) = (i64::MAX, i64::MAX, -1i32);
        for (k, &base) in row.iter().enumerate() {
            let add = if k as i32 == prev_idx { prev_min2 } else { prev_min1 };
            let c = base + add;
            if c < cur_min1 {
                cur_min2 = cur_min1;
                cur_min1 = c;
                cur_idx = k as i32;
            } else if c < cur_min2 {
                cur_min2 = c;
            }
        }
        prev_min1 = cur_min1;
        prev_min2 = cur_min2;
        prev_idx = cur_idx;
    }
    prev_min1
}

fn main() {
    let costs = vec![vec![1, 5, 3], vec![2, 9, 4]];
    println!("{}", min_cost(&costs)); // expected 5
}
