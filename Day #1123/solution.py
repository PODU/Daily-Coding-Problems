# Day 1123: Day 1123 - Largest divisible subset
# Approach: sort, then LIS-style DP where j extends i if a[i] % a[j] == 0;
# reconstruct via parent pointers. Time: O(n^2), Space: O(n).

def largest_divisible_subset(nums):
    if not nums:
        return []
    nums = sorted(nums)
    n = len(nums)
    dp = [1] * n
    parent = [-1] * n
    best = 0
    for i in range(n):
        for j in range(i):
            if nums[i] % nums[j] == 0 and dp[j] + 1 > dp[i]:
                dp[i] = dp[j] + 1
                parent[i] = j
        if dp[i] > dp[best]:
            best = i
    res = []
    k = best
    while k != -1:
        res.append(nums[k])
        k = parent[k]
    return res[::-1]


if __name__ == "__main__":
    print(largest_divisible_subset([3, 5, 10, 20, 21]))  # [5, 10, 20]
    print(largest_divisible_subset([1, 3, 6, 24]))       # [1, 3, 6, 24]
