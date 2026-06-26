// h-index via counting sort: bucket citations capped at N, scan from high to low
// accumulating papers until count >= citation level. Time O(N), Space O(N).

fn h_index(citations: &[i32]) -> i32 {
    let n = citations.len();
    let mut bucket = vec![0i32; n + 1];
    for &c in citations {
        let idx = (c as usize).min(n);
        bucket[idx] += 1;
    }
    let mut acc = 0;
    for h in (0..=n).rev() {
        acc += bucket[h];
        if acc >= h as i32 {
            return h as i32;
        }
    }
    0
}

fn main() {
    let citations = [4, 3, 0, 1, 5];
    println!("{}", h_index(&citations));
}
