// Day 864: Minimum rescue boats (<=2 people, total weight <= k).
// Approach: sort, greedily pair lightest with heaviest using two pointers.
// Time: O(n log n), Space: O(1).
fn num_rescue_boats(mut weights: Vec<i32>, k: i32) -> i32 {
    weights.sort();
    let mut i = 0i32;
    let mut j = weights.len() as i32 - 1;
    let mut boats = 0;
    while i <= j {
        if weights[i as usize] + weights[j as usize] <= k {
            i += 1;
        }
        j -= 1;
        boats += 1;
    }
    boats
}

fn main() {
    println!("{}", num_rescue_boats(vec![100, 200, 150, 80], 200)); // 3
}
