// Day 1105: Partition multiset into two equal-sum subsets (subset-sum DP).
// If total odd -> false; else can we reach total/2. Time: O(N*sum). Space: O(sum).
fn can_partition(nums: &[i32]) -> bool {
    let total: i32 = nums.iter().sum();
    if total % 2 == 1 {
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
    println!("{}", can_partition(&[15, 5, 20, 10, 35, 15, 10])); // true
    println!("{}", can_partition(&[15, 5, 20, 10, 35]));         // false
}
