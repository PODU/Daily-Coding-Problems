// LIS length via patience sorting: maintain tails[], binary search (partition_point/lower_bound) for strict increase. O(n log n) time, O(n) space.
fn length_of_lis(nums: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::new();
    for &x in nums {
        let i = tails.partition_point(|&t| t < x);
        if i == tails.len() {
            tails.push(x);
        } else {
            tails[i] = x;
        }
    }
    tails.len()
}

fn main() {
    let nums = [10, 9, 2, 5, 3, 7, 101, 18];
    println!("{}", length_of_lis(&nums));
}
