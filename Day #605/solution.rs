// h-index via counting/bucket sort: bucket citations capped at N, scan from high
// to low accumulating paper counts. Time O(n), Space O(n).
fn h_index(citations: &[usize]) -> usize {
    let n = citations.len();
    let mut bucket = vec![0usize; n + 1];
    for &c in citations {
        bucket[c.min(n)] += 1;
    }
    let mut total = 0usize;
    for h in (0..=n).rev() {
        total += bucket[h];
        if total >= h {
            return h;
        }
    }
    0
}

fn main() {
    let citations = vec![4, 3, 0, 1, 5];
    println!("{}", h_index(&citations));
}
