# Day 1636: Can a string be made a palindrome by deleting at most k chars.
# min deletions = n - LongestPalindromicSubsequence; DP O(n^2) time/space.


def can_make_palindrome(s: str, k: int) -> bool:
    n = len(s)
    if n == 0:
        return True
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
    lps = dp[0][n - 1]
    return (n - lps) <= k


if __name__ == "__main__":
    print("True" if can_make_palindrome("waterrfetawx", 2) else "False")
