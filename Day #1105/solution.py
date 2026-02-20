# Day 1105: Partition multiset into two equal-sum subsets (subset-sum DP).
# If total odd -> false; else can we reach total/2. Time: O(N*sum). Space: O(sum).
def can_partition(nums):
    total = sum(nums)
    if total % 2 == 1:
        return False
    target = total // 2
    dp = [False] * (target + 1)
    dp[0] = True
    for x in nums:
        for s in range(target, x - 1, -1):
            if dp[s - x]:
                dp[s] = True
    return dp[target]


if __name__ == "__main__":
    print(can_partition([15, 5, 20, 10, 35, 15, 10]))  # True
    print(can_partition([15, 5, 20, 10, 35]))          # False
