# Day 464: Largest divisible subset.
# Approach: sort, then LIS-style DP where dp[i]=longest chain ending at i;
# j divides i means a[i] % a[j] == 0. Reconstruct via parent pointers.
# Time: O(n^2), Space: O(n).


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
    i = best
    while i >= 0:
        res.append(nums[i])
        i = parent[i]
    return res[::-1]


if __name__ == "__main__":
    print(largest_divisible_subset([3, 5, 10, 20, 21]))
    print(largest_divisible_subset([1, 3, 6, 24]))
