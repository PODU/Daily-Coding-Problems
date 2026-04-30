// Day 1448: Fewest bricks cut by a vertical line through a brick wall.
// Count internal edge positions via prefix sums; answer = rows - maxEdgeCount.
// Time O(total bricks), Space O(distinct edges).
use std::collections::HashMap;

fn least_bricks(wall: &[Vec<i64>]) -> usize {
    let mut edges: HashMap<i64, usize> = HashMap::new();
    let mut best = 0;
    for row in wall {
        let mut pos = 0i64;
        for i in 0..row.len().saturating_sub(1) {
            // skip the far right edge
            pos += row[i];
            let c = edges.entry(pos).or_insert(0);
            *c += 1;
            best = best.max(*c);
        }
    }
    wall.len() - best
}

fn main() {
    let wall = vec![
        vec![3, 5, 1, 1],
        vec![2, 3, 3, 2],
        vec![5, 5],
        vec![4, 4, 2],
        vec![1, 3, 3, 3],
        vec![1, 1, 6, 1, 1],
    ];
    println!("{}", least_bricks(&wall)); // 2
}
