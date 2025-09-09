// h-index via counting/bucket sort. Time O(N), Space O(N).
// Bucket counts of citations capped at N, then scan from high to low.
fn h_index(citations: &[i32]) -> i32 {
    let n = citations.len();
    let mut bucket = vec![0usize; n + 1];
    for &c in citations {
        bucket[(c as usize).min(n)] += 1;
    }
    let mut total = 0usize;
    for h in (0..=n).rev() {
        total += bucket[h];
        if total >= h {
            return h as i32;
        }
    }
    0
}

fn main() {
    let citations = [4, 3, 0, 1, 5];
    println!("{}", h_index(&citations));
}
