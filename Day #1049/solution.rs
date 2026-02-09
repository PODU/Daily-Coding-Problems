// LSD radix sort (base 256, 4 passes over 32-bit ints). O(N*d)~O(N) time, O(N) space.
// Avoids a 1e9-size counting array; memory scales with N, not value range. Else: external merge sort.

fn radix_sort(mut a: Vec<u32>) -> Vec<u32> {
    let n = a.len();
    let mut buf = vec![0u32; n];
    let mut shift = 0;
    while shift < 32 {
        let mut count = [0usize; 256];
        for &v in &a {
            count[((v >> shift) & 0xFF) as usize] += 1;
        }
        let mut sum = 0;
        for i in 0..256 {
            let c = count[i];
            count[i] = sum;
            sum += c;
        }
        for &v in &a {
            let d = ((v >> shift) & 0xFF) as usize;
            buf[count[d]] = v;
            count[d] += 1;
        }
        std::mem::swap(&mut a, &mut buf);
        shift += 8;
    }
    a
}

fn main() {
    let a = vec![829, 3, 1000000000u32, 42, 17, 999, 256, 0, 524287, 42];
    let a = radix_sort(a);
    let parts: Vec<String> = a.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
