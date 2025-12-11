// Day 732: Minimum boats (each holds <=2 people, weight limit k).
// Approach: Sort; two pointers pair lightest with heaviest when they fit.
// Time: O(n log n), Space: O(1).

fn num_boats(mut w: Vec<i32>, k: i32) -> i32 {
    w.sort();
    let (mut i, mut j, mut boats) = (0i32, w.len() as i32 - 1, 0);
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
    println!("{}", num_boats(vec![100, 200, 150, 80], 200)); // 3
}
