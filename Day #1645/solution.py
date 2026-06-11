# Day 1645: Regex full-match with '.' and '*' via DP: dp[i][j] = s[0..i) matches p[0..j).
# '*' uses zero-copy (dp[i][j-2]) or one-more (prev char match). Time/Space O(m*n).

def is_match(s: str, p: str) -> bool:
    m, n = len(s), len(p)
    dp = [[False] * (n + 1) for _ in range(m + 1)]
    dp[0][0] = True
    for j in range(1, n + 1):
        if p[j - 1] == '*' and j >= 2:
            dp[0][j] = dp[0][j - 2]
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if p[j - 1] == '*':
                dp[i][j] = j >= 2 and dp[i][j - 2]
                if j >= 2 and p[j - 2] in ('.', s[i - 1]):
                    dp[i][j] = dp[i][j] or dp[i - 1][j]
            elif p[j - 1] in ('.', s[i - 1]):
                dp[i][j] = dp[i - 1][j - 1]
    return dp[m][n]


if __name__ == "__main__":
    print(str(is_match("ray", "ra.")).lower())
    print(str(is_match("raymond", "ra.")).lower())
    print(str(is_match("chat", ".*at")).lower())
    print(str(is_match("chats", ".*at")).lower())
