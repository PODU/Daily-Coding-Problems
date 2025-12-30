// First missing positive via cyclic sort: place nums[i] at index nums[i]-1, then scan. O(n) time, O(1) space.

fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut i = 0;
    while i < n {
        let v = nums[i];
        if v > 0 && (v as usize) <= n && nums[v as usize - 1] != v {
            nums.swap(i, v as usize - 1);
        } else {
            i += 1;
        }
    }
    for i in 0..n {
        if nums[i] != i as i32 + 1 {
            return i as i32 + 1;
        }
    }
    n as i32 + 1
}

fn main() {
    println!("{}", first_missing_positive(vec![3, 4, -1, 1]));
    println!("{}", first_missing_positive(vec![1, 2, 0]));
}
