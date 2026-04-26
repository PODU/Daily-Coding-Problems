# Day 1428: Partition array into two subsets minimizing sum difference.
# Approach: subset-sum DP over half the total; reconstruct one subset.
# Time: O(n * sum), Space: O(n * sum).

def min_subset_diff(a):
    n = len(a)
    total = sum(a)
    half = total // 2
    dp = [[False] * (half + 1) for _ in range(n + 1)]
    for i in range(n + 1):
        dp[i][0] = True
    for i in range(1, n + 1):
        for s in range(half + 1):
            dp[i][s] = dp[i - 1][s]
            if s >= a[i - 1] and dp[i - 1][s - a[i - 1]]:
                dp[i][s] = True

    best = 0
    for s in range(half, -1, -1):
        if dp[n][s]:
            best = s
            break

    subset = []
    s = best
    for i in range(n, 0, -1):
        if s >= a[i - 1] and dp[i - 1][s - a[i - 1]]:
            subset.append(a[i - 1])
            s -= a[i - 1]

    return total - 2 * best, subset


if __name__ == "__main__":
    diff, subset = min_subset_diff([5, 10, 15, 20, 25])
    print("Difference:", diff)   # Difference: 5
    print("Subset:", subset)
