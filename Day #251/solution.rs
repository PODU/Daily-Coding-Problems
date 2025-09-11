// External/bucket sort demo for sorting ~1M ints in [0,1e9] with bounded memory.
// Real answer for data exceeding RAM: external merge sort (split into chunks ->
// sort each chunk on disk -> k-way merge). Here we bucket by range, sort each
// bucket, and concatenate. Time: O(n log(n/k)); Space: O(n) bounded per bucket.

fn bucket_sort(data: &[i64], max_val: i64, num_buckets: usize) -> Vec<i64> {
    let width = max_val / num_buckets as i64 + 1;
    let mut buckets: Vec<Vec<i64>> = vec![Vec::new(); num_buckets];
    for &x in data {
        let mut b = (x / width) as usize;
        if b >= num_buckets {
            b = num_buckets - 1;
        }
        buckets[b].push(x);
    }
    let mut out = Vec::with_capacity(data.len());
    for bk in buckets.iter_mut() {
        bk.sort(); // each bucket fits in memory
        out.extend(bk.iter().copied());
    }
    out
}

fn main() {
    let input = [5i64, 1, 4, 2, 8, 1000000000, 0];
    let sorted = bucket_sort(&input, 1000000000, 16);
    let parts: Vec<String> = sorted.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
