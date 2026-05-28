# Day 1580: Largest divisible subset (every pair mutually divisible).
# Sort, then LIS-style DP: dp[i] = longest chain ending at i where a[i] % a[j] == 0.
# Time: O(n^2); Space: O(n).


def largest_divisible(a):
    a = sorted(a)
    n = len(a)
    if n == 0:
        return []
    dp = [1] * n
    prev = [-1] * n
    best = 0
    for i in range(n):
        for j in range(i):
            if a[i] % a[j] == 0 and dp[j] + 1 > dp[i]:
                dp[i] = dp[j] + 1
                prev[i] = j
        if dp[i] > dp[best]:
            best = i
    res = []
    i = best
    while i != -1:
        res.append(a[i])
        i = prev[i]
    return res[::-1]


if __name__ == "__main__":
    print(largest_divisible([3, 5, 10, 20, 21]))  # [5, 10, 20]
