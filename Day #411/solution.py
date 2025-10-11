# Day 411: Can we make s a palindrome by deleting at most k chars?
# Answer: n - LPS(s) <= k, where LPS = longest palindromic subsequence via DP. O(n^2) time, O(n^2) space.
def lps(s):
    n = len(s)
    dp = [[0] * n for _ in range(n)]
    for i in range(n - 1, -1, -1):
        dp[i][i] = 1
        for j in range(i + 1, n):
            if s[i] == s[j]:
                dp[i][j] = dp[i + 1][j - 1] + 2
            else:
                dp[i][j] = max(dp[i + 1][j], dp[i][j - 1])
    return dp[0][n - 1]


def can_make_palindrome(s, k):
    return len(s) - lps(s) <= k


if __name__ == "__main__":
    s = "waterrfetawx"
    k = 2
    print(can_make_palindrome(s, k))
