// Day 1141: Min cost to pack people (remove gaps).
// Transform p_i -> p_i - i, answer = sum |q_i - median(q)|. Time O(n log n), Space O(m).
fn min_cost(seats: &[i32]) -> i64 {
    let mut q: Vec<i64> = Vec::new();
    let mut idx = 0i64;
    for (i, &v) in seats.iter().enumerate() {
        if v == 1 {
            q.push(i as i64 - idx);
            idx += 1;
        }
    }
    if q.is_empty() {
        return 0;
    }
    q.sort();
    let med = q[q.len() / 2];
    q.iter().map(|&v| (v - med).abs()).sum()
}

fn main() {
    println!("{}", min_cost(&[0, 1, 1, 0, 1, 0, 0, 0, 1])); // 5
}
