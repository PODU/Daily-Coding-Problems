// Couples holding hands: union the two couples occupying each seat-pair. Minimum swaps =
// N - (number of connected components among couples). Time: O(N alpha), Space: O(N).
fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    let mut x = x;
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

fn min_swaps(row: &[usize]) -> usize {
    let n = row.len() / 2;
    let mut parent: Vec<usize> = (0..n).collect();
    let mut comps = n;
    let mut i = 0;
    while i < row.len() {
        let a = find(&mut parent, row[i] / 2);
        let b = find(&mut parent, row[i + 1] / 2);
        if a != b {
            parent[a] = b;
            comps -= 1;
        }
        i += 2;
    }
    n - comps
}

fn main() {
    println!("{}", min_swaps(&[0, 2, 1, 3])); // 1
}
