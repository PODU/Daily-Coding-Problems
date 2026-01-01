// Minimum swaps to seat N couples side by side.
// Greedy: partner of p is p^1; swap mismatched partner into place. Time: O(N), Space: O(N).

fn min_swaps(row: &[i32]) -> i32 {
    let n = row.len();
    let mut r = row.to_vec();
    let mut pos = vec![0usize; n];
    for (i, &v) in r.iter().enumerate() {
        pos[v as usize] = i;
    }
    let mut swaps = 0;
    let mut i = 0;
    while i < n {
        let partner = r[i] ^ 1;
        if r[i + 1] != partner {
            let j = pos[partner as usize];
            pos[r[i + 1] as usize] = j;
            pos[r[j] as usize] = i + 1;
            r.swap(i + 1, j);
            swaps += 1;
        }
        i += 2;
    }
    swaps
}

fn main() {
    let row = [0, 2, 1, 3];
    println!("{}", min_swaps(&row));
}
