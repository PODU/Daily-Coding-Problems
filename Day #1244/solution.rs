// Boats: sort, greedily pair lightest+heaviest (two-pointer). Time O(n log n).
fn num_boats(weights: &[i32], k: i32) -> i32 {
    let mut w = weights.to_vec();
    w.sort();
    let mut i = 0i32;
    let mut j = w.len() as i32 - 1;
    let mut boats = 0;
    while i <= j {
        if w[i as usize] + w[j as usize] <= k {
            i += 1;
        }
        j -= 1;
        boats += 1;
    }
    boats
}

fn main() {
    println!("{}", num_boats(&[100, 200, 150, 80], 200));
}
