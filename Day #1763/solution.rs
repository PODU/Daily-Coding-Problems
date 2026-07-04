// Day 1763: Sort ~1e6 ints in range [0,1e9]. The MILLION elements fit in memory
// (~4MB); only the billion VALUE range is large. Use LSD radix sort (base 256,
// 4 passes) — O(n) time, O(n) space, optimal for fixed-width integers.
// If even the million don't fit, fall back to external merge sort (chunk-sort to
// disk, then k-way merge).
fn radix_sort(input: &[u32]) -> Vec<u32> {
    let mut a = input.to_vec();
    let mut tmp = vec![0u32; a.len()];
    let mut shift = 0;
    while shift < 32 {
        let mut count = [0usize; 256];
        for &x in &a {
            count[((x >> shift) & 0xFF) as usize] += 1;
        }
        let mut sum = 0;
        for c in count.iter_mut() {
            let cur = *c;
            *c = sum;
            sum += cur;
        }
        for &x in &a {
            let d = ((x >> shift) & 0xFF) as usize;
            tmp[count[d]] = x;
            count[d] += 1;
        }
        std::mem::swap(&mut a, &mut tmp);
        shift += 8;
    }
    a
}

fn main() {
    let data = [999999999u32, 0, 123456789, 42, 1000000000, 7, 500000000];
    let sorted = radix_sort(&data);
    let parts: Vec<String> = sorted.iter().map(|x| x.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
