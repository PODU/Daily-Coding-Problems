# Day 121: Can form palindrome by deleting at most k chars.
# Min deletions = n - LongestPalindromicSubsequence. DP O(n^2) time, O(n^2) space.


def lps(s):
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
                dp[i][j] = 2 + (dp[i + 1][j - 1] if length > 2 else 0)
            else:
                dp[i][j] = max(dp[i + 1][j], dp[i][j - 1])
    return dp[0][n - 1]


def can_make_palindrome(s, k):
    return len(s) - lps(s) <= k


if __name__ == "__main__":
    s, k = "waterrfetawx", 2
    if can_make_palindrome(s, k):
        print("You could delete f and x to get 'waterretaw'.")
    else:
        print("Not possible")
