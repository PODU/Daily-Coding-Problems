// Boats: <=2 people, weight limit k, minimize boats. Sort + two pointers.
// Pair lightest with heaviest if they fit, else heaviest alone. Time O(n log n), Space O(1).
fn num_boats(weights: &[i32], k: i32) -> i32 {
    let mut w = weights.to_vec();
    w.sort();
    let mut l = 0i32;
    let mut h = w.len() as i32 - 1;
    let mut boats = 0;
    while l <= h {
        if w[l as usize] + w[h as usize] <= k {
            l += 1;
        }
        h -= 1;
        boats += 1;
    }
    boats
}

fn main() {
    println!("{}", num_boats(&[100, 200, 150, 80], 200));
}
