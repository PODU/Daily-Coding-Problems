// Brick wall: hashmap of prefix-sum edge positions (excluding full-width edge).
// Answer = numRows - maxEdgeCount. Time O(total bricks), Space O(distinct edges).
use std::collections::HashMap;

fn least_bricks(wall: &Vec<Vec<i32>>) -> i32 {
    let mut edges: HashMap<i64, i32> = HashMap::new();
    let mut max_count = 0;
    for row in wall {
        let mut sum: i64 = 0;
        for i in 0..row.len() - 1 {
            sum += row[i] as i64;
            let c = edges.entry(sum).or_insert(0);
            *c += 1;
            if *c > max_count {
                max_count = *c;
            }
        }
    }
    wall.len() as i32 - max_count
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
    println!("{}", least_bricks(&wall));
}
