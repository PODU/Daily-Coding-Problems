# Day 1380: Subset sum returning an actual subset via DP + backtracking reconstruction.
# Time O(n*k), Space O(n*k). Returns None if no subset sums to k.


def subset_sum(s, k):
    n = len(s)
    dp = [[False] * (k + 1) for _ in range(n + 1)]
    for i in range(n + 1):
        dp[i][0] = True
    for i in range(1, n + 1):
        for j in range(k + 1):
            dp[i][j] = dp[i - 1][j]
            if j >= s[i - 1] and dp[i - 1][j - s[i - 1]]:
                dp[i][j] = True
    if not dp[n][k]:
        return None
    res, j = [], k
    for i in range(n, 0, -1):
        if not dp[i - 1][j]:
            res.append(s[i - 1])
            j -= s[i - 1]
    res.reverse()
    return res


if __name__ == "__main__":
    print(subset_sum([12, 1, 61, 5, 9, 2], 24))
