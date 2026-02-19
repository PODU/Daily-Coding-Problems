// Day 1102: Min total movement to seat people contiguously (order preserved).
// person i lands at start+i; minimize sum|pos[i]-(start+i)| => shift b[i]=pos[i]-i
// to its median. Time: O(N). Space: O(M).
fn min_cost(seats: &[i32]) -> i64 {
    let mut b = Vec::new();
    let mut p = 0i64;
    for (i, &s) in seats.iter().enumerate() {
        if s == 1 {
            b.push(i as i64 - p);
            p += 1;
        }
    }
    if b.is_empty() {
        return 0;
    }
    b.sort();
    let med = b[b.len() / 2];
    b.iter().map(|&x| (x - med).abs()).sum()
}

fn main() {
    println!("{}", min_cost(&[0, 1, 1, 0, 1, 0, 0, 0, 1])); // 5
}
