// Day 607: Min total movement to seat M people contiguously in a row.
// Approach: target = median of (pos[i]-i); cost = sum |(pos[i]-i) - median|. Time O(n), Space O(M).

fn min_cost(seats: &[i32]) -> i64 {
    let mut b: Vec<i64> = Vec::new();
    let mut idx = 0i64;
    for (i, &s) in seats.iter().enumerate() {
        if s == 1 {
            b.push(i as i64 - idx);
            idx += 1;
        }
    }
    if b.is_empty() {
        return 0;
    }
    b.sort();
    let med = b[b.len() / 2];
    b.iter().map(|&v| (v - med).abs()).sum()
}

fn main() {
    let seats = [0, 1, 1, 0, 1, 0, 0, 0, 1];
    println!("{}", min_cost(&seats)); // 5
}
