# Day 647: Partition equal subset sum: if total odd -> false, else subset-sum DP for sum/2.
# Time O(n*sum), Space O(sum).
def can_partition(nums):
    total = sum(nums)
    if total % 2 != 0:
        return False
    target = total // 2
    dp = [False] * (target + 1)
    dp[0] = True
    for x in nums:
        for j in range(target, x - 1, -1):
            if dp[j - x]:
                dp[j] = True
    return dp[target]


if __name__ == "__main__":
    a = [15, 5, 20, 10, 35, 15, 10]
    b = [15, 5, 20, 10, 35]
    print("true" if can_partition(a) else "false")
    print("true" if can_partition(b) else "false")
