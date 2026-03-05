// H-index via counting buckets: bucket papers by min(citation, n), scan from high. O(n) time, O(n) space.
fn h_index(citations: &[i32]) -> i32 {
    let n = citations.len();
    let mut bucket = vec![0usize; n + 1];
    for &c in citations {
        let idx = std::cmp::min(c as usize, n);
        bucket[idx] += 1;
    }
    let mut count = 0usize;
    for h in (0..=n).rev() {
        count += bucket[h];
        if count >= h {
            return h as i32;
        }
    }
    0
}

fn main() {
    let citations = [4, 3, 0, 1, 5];
    println!("{}", h_index(&citations));
}
