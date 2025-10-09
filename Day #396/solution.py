# Day 396: Longest palindromic subsequence via interval DP dp[i][j] over s[i..j].
# Time O(n^2), Space O(n^2).
def lps(s: str) -> int:
    n = len(s)
    if n == 0:
        return 0
    dp = [[0] * n for _ in range(n)]
    for i in range(n):
        dp[i][i] = 1
    for length in range(2, n + 1):
        for i in range(0, n - length + 1):
            j = i + length - 1
            if s[i] == s[j]:
                dp[i][j] = 2 + (0 if length == 2 else dp[i + 1][j - 1])
            else:
                dp[i][j] = max(dp[i + 1][j], dp[i][j - 1])
    return dp[0][n - 1]


if __name__ == "__main__":
    print(lps("MAPTPTMTPA"))
