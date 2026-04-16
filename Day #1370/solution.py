# Day 1370: Can form palindrome by deleting <= k chars: min deletions = n - LPS(s).
# LPS via DP. Time O(n^2), Space O(n^2).


def lps(s):
    n = len(s)
    dp = [[0] * n for _ in range(n)]
    for i in range(n):
        dp[i][i] = 1
    for length in range(2, n + 1):
        for i in range(n - length + 1):
            j = i + length - 1
            if s[i] == s[j]:
                dp[i][j] = 2 if length == 2 else dp[i + 1][j - 1] + 2
            else:
                dp[i][j] = max(dp[i + 1][j], dp[i][j - 1])
    return dp[0][n - 1]


def can_make_palindrome(s, k):
    return len(s) - lps(s) <= k


if __name__ == "__main__":
    print(can_make_palindrome("waterrfetawx", 2))
