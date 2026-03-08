// Day 1175: Maximum subarray sum in a circular array (empty allowed -> 0).
// Answer = max(0, kadaneMax, total - kadaneMin); total-min covers the wrap case.
// Time O(N), Space O(1).

fn max_circular_subarray(a: &[i64]) -> i64 {
    let (mut total, mut cur_max, mut best_max, mut cur_min, mut best_min) = (0i64, 0i64, 0i64, 0i64, 0i64);
    for &x in a {
        total += x;
        cur_max = x.max(cur_max + x);
        best_max = best_max.max(cur_max);
        cur_min = x.min(cur_min + x);
        best_min = best_min.min(cur_min);
    }
    0.max(best_max.max(total - best_min))
}

fn main() {
    println!("{}", max_circular_subarray(&[8, -1, 3, 4])); // 15
    println!("{}", max_circular_subarray(&[-4, 5, 1, 0])); // 6
}
