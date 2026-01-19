// Brick wall: count edge offsets (cumulative sums excluding last) via hashmap.
// Fewest cuts = numRows - maxAlignedEdges. Time O(total bricks), Space O(distinct edges).
use std::collections::HashMap;

fn least_bricks(wall: &[Vec<i64>]) -> i64 {
    let mut freq: HashMap<i64, i64> = HashMap::new();
    let mut best = 0;
    for row in wall {
        let mut sum = 0i64;
        for i in 0..row.len().saturating_sub(1) {
            sum += row[i];
            let c = freq.entry(sum).or_insert(0);
            *c += 1;
            if *c > best {
                best = *c;
            }
        }
    }
    wall.len() as i64 - best
}

fn main() {
    let wall: Vec<Vec<i64>> = vec![
        vec![3, 5, 1, 1],
        vec![2, 3, 3, 2],
        vec![5, 5],
        vec![4, 4, 2],
        vec![1, 3, 3, 3],
        vec![1, 1, 6, 1, 1],
    ];
    println!("{}", least_bricks(&wall));
}
