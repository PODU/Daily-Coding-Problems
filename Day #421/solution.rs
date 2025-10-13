// Day 421: LSD radix sort (base-256). O(n*w) time, O(n) space.
// 4 byte-passes for 32-bit ints; no billion-element array needed.
fn radix_sort(mut a: Vec<u32>) -> Vec<u32> {
    let n = a.len();
    let mut buf = vec![0u32; n];
    let mut shift = 0;
    while shift < 32 {
        let mut cnt = [0usize; 257];
        for &x in &a {
            cnt[(((x >> shift) & 0xFF) as usize) + 1] += 1;
        }
        for i in 0..256 {
            cnt[i + 1] += cnt[i];
        }
        for &x in &a {
            let d = ((x >> shift) & 0xFF) as usize;
            buf[cnt[d]] = x;
            cnt[d] += 1;
        }
        std::mem::swap(&mut a, &mut buf);
        shift += 8;
    }
    a
}

fn main() {
    let a = vec![5u32, 3, 1000000000, 0, 42, 7, 42];
    let s = radix_sort(a);
    let parts: Vec<String> = s.iter().map(|x| x.to_string()).collect();
    println!("Sorted: [{}]", parts.join(", "));
}
