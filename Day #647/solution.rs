// Partition equal subset sum: if total odd -> false, else subset-sum DP for sum/2.
// Time O(n*sum), Space O(sum).
fn can_partition(nums: &[i32]) -> bool {
    let total: i32 = nums.iter().sum();
    if total % 2 != 0 {
        return false;
    }
    let target = (total / 2) as usize;
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for &x in nums {
        let x = x as usize;
        for j in (x..=target).rev() {
            if dp[j - x] {
                dp[j] = true;
            }
        }
    }
    dp[target]
}

fn main() {
    let a = [15, 5, 20, 10, 35, 15, 10];
    let b = [15, 5, 20, 10, 35];
    println!("{}", can_partition(&a));
    println!("{}", can_partition(&b));
}
