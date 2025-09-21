// Day 309: Min movement to pack people with no gaps. Map p_i - i; cost minimized
// at the median of those values. O(M log M).
fn min_cost(seats: &[i32]) -> i64 {
    let positions: Vec<usize> = seats.iter().enumerate().filter(|(_, &v)| v == 1).map(|(i, _)| i).collect();
    if positions.is_empty() { return 0; }
    let mut b: Vec<i64> = positions.iter().enumerate().map(|(i, &p)| p as i64 - i as i64).collect();
    b.sort();
    let med = b[b.len() / 2];
    b.iter().map(|x| (x - med).abs()).sum()
}

fn main() {
    println!("{}", min_cost(&[0, 1, 1, 0, 1, 0, 0, 0, 1])); // 5
}
