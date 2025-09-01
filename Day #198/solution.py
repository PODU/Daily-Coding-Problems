# Day 198: Largest divisible subset.
# Sort, then DP: dp[i] = longest chain ending at i (a[j] | a[i]); track parent for reconstruction.
# Time: O(n^2), Space: O(n).


def largest_divisible_subset(a):
    n = len(a)
    if n == 0:
        return []
    a = sorted(a)
    dp = [1] * n
    parent = [-1] * n
    best = 0
    for i in range(n):
        for j in range(i):
            if a[i] % a[j] == 0 and dp[j] + 1 > dp[i]:
                dp[i] = dp[j] + 1
                parent[i] = j
        if dp[i] > dp[best]:
            best = i
    res = []
    i = best
    while i != -1:
        res.append(a[i])
        i = parent[i]
    return res[::-1]


if __name__ == "__main__":
    print(largest_divisible_subset([3, 5, 10, 20, 21]))  # [5, 10, 20]
    print(largest_divisible_subset([1, 3, 6, 24]))        # [1, 3, 6, 24]
