// Boat rescue: min boats, <=2 people each, weight limit k.
// Greedy two-pointer: sort, pair lightest with heaviest if sum<=k. O(n log n) time, O(1) extra.
fn num_rescue_boats(mut w: Vec<i32>, k: i32) -> i32 {
    w.sort();
    let mut lo: isize = 0;
    let mut hi: isize = w.len() as isize - 1;
    let mut boats = 0;
    while lo <= hi {
        if w[lo as usize] + w[hi as usize] <= k {
            lo += 1;
        }
        hi -= 1;
        boats += 1;
    }
    boats
}

fn main() {
    println!("{}", num_rescue_boats(vec![100, 200, 150, 80], 200)); // expected 3
}
