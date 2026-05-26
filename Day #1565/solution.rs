// Subset-sum DP: partition into equal halves iff target=sum/2 reachable. Time O(n*sum), Space O(sum).
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
        for s in (x..=target).rev() {
            if dp[s - x] {
                dp[s] = true;
            }
        }
    }
    dp[target]
}

fn main() {
    let nums = [15, 5, 20, 10, 35, 15, 10];
    println!("{}", if can_partition(&nums) { "true" } else { "false" });
}
