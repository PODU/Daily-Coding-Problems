// Day 1202: Minimum swaps so couples sit side by side.
// Greedy: for each even seat, swap partner of its occupant into the next seat. Time O(N), Space O(N).
fn min_swaps(mut row: Vec<usize>) -> usize {
    let n = row.len();
    let mut pos = vec![0usize; n];
    for (i, &v) in row.iter().enumerate() {
        pos[v] = i;
    }
    let mut swaps = 0;
    let mut i = 0;
    while i < n {
        let partner = row[i] ^ 1;
        if row[i + 1] != partner {
            let j = pos[partner];
            let (a, b) = (row[i + 1], row[j]);
            pos.swap(a, b);
            row.swap(i + 1, j);
            swaps += 1;
        }
        i += 2;
    }
    swaps
}

fn main() {
    println!("{}", min_swaps(vec![0, 2, 1, 3])); // 1
}
