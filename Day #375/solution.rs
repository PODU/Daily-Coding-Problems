// Day 375: h-index via counting sort.
// Bucket citations (capped at n), then scan h from n down accumulating papers
// with >= h citations; first h with count >= h wins. Time O(n), Space O(n).

fn h_index(cites: &[i32]) -> i32 {
    let n = cites.len();
    let mut buckets = vec![0i32; n + 1];
    for &c in cites {
        buckets[(c as usize).min(n)] += 1;
    }
    let mut total = 0;
    for h in (0..=n).rev() {
        total += buckets[h];
        if total >= h as i32 {
            return h as i32;
        }
    }
    0
}

fn main() {
    println!("{}", h_index(&[4, 0, 0, 2, 3])); // 2
}
