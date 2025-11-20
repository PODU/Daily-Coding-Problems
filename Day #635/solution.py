# Day 635: Regular expression matching with '.' and '*' (full-string match).
# Approach: bottom-up DP; dp[i][j] = does s[i:] match p[j:].
# Time: O(m*n), Space: O(m*n).
def is_match(s, p):
    m, n = len(s), len(p)
    dp = [[False] * (n + 1) for _ in range(m + 1)]
    dp[m][n] = True
    for i in range(m, -1, -1):
        for j in range(n - 1, -1, -1):
            first = i < m and (p[j] == s[i] or p[j] == '.')
            if j + 1 < n and p[j + 1] == '*':
                dp[i][j] = dp[i][j + 2] or (first and dp[i + 1][j])
            else:
                dp[i][j] = first and dp[i + 1][j + 1]
    return dp[0][0]


if __name__ == "__main__":
    print(str(is_match("ray", "ra.")).lower())      # true
    print(str(is_match("raymond", "ra.")).lower())  # false
    print(str(is_match("chat", ".*at")).lower())    # true
    print(str(is_match("chats", ".*at")).lower())   # false
