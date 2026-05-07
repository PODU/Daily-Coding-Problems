// Day 1480: Sort a million ints in [0, 1e9] without a billion-element array.
// Index by element count, not by value. LSD radix sort (base 256) is O(N).
// For out-of-core data the same idea generalizes to external merge sort.
// Time O(N) (4 passes for 32-bit), Space O(N).

fn radix_sort(input: &[u32]) -> Vec<u32> {
    if input.is_empty() {
        return Vec::new();
    }
    let mut out = input.to_vec();
    let mut tmp = vec![0u32; out.len()];
    let mut shift = 0;
    while shift < 32 {
        let mut count = [0usize; 257];
        for &v in &out {
            count[(((v >> shift) & 0xFF) as usize) + 1] += 1;
        }
        for i in 0..256 {
            count[i + 1] += count[i];
        }
        for &v in &out {
            let d = ((v >> shift) & 0xFF) as usize;
            tmp[count[d]] = v;
            count[d] += 1;
        }
        std::mem::swap(&mut out, &mut tmp);
        shift += 8;
    }
    out
}

fn main() {
    println!("{:?}", radix_sort(&[9, 11, 8, 5, 7, 10])); // [5, 7, 8, 9, 10, 11]
}
