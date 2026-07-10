// Collect positions p_i of people, set q_i = p_i - i, answer = sum |q_i - median(q)|.
// Time O(n), Space O(m).
fn min_cost(seats: &[i32]) -> i64 {
    let mut q: Vec<i64> = Vec::new();
    let mut i: i64 = 0;
    for (j, &s) in seats.iter().enumerate() {
        if s == 1 {
            q.push(j as i64 - i);
            i += 1;
        }
    }
    if q.is_empty() {
        return 0;
    }
    let med = q[q.len() / 2];
    q.iter().map(|&v| (v - med).abs()).sum()
}

fn main() {
    let seats = [0, 1, 1, 0, 1, 0, 0, 0, 1];
    println!("{}", min_cost(&seats)); // expected 5
}
