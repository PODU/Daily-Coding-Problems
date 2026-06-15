// Brick wall: prefix-sum edge positions per row, max edge frequency => fewest cuts = rows - maxEdges. O(total bricks) time/space.
use std::collections::HashMap;
fn main() {
    let wall = vec![
        vec![3, 5, 1, 1],
        vec![2, 3, 3, 2],
        vec![5, 5],
        vec![4, 4, 2],
        vec![1, 3, 3, 3],
        vec![1, 1, 6, 1, 1],
    ];
    let mut freq: HashMap<i64, i32> = HashMap::new();
    let mut best = 0;
    for row in &wall {
        let mut sum: i64 = 0;
        for i in 0..row.len() - 1 {
            sum += row[i] as i64;
            let c = freq.entry(sum).or_insert(0);
            *c += 1;
            if *c > best { best = *c; }
        }
    }
    println!("{}", wall.len() as i32 - best);
}
