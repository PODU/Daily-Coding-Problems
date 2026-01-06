# Day 860: Regex matching with '.' and '*'.
# Approach: bottom-up DP, dp[i][j] = does s[i:] match p[j:].
# Time: O(n*m), Space: O(n*m).


def is_match(s, p):
    n, m = len(s), len(p)
    dp = [[False] * (m + 1) for _ in range(n + 1)]
    dp[n][m] = True
    for i in range(n, -1, -1):
        for j in range(m - 1, -1, -1):
            first = i < n and p[j] in (s[i], '.')
            if j + 1 < m and p[j + 1] == '*':
                dp[i][j] = dp[i][j + 2] or (first and dp[i + 1][j])
            else:
                dp[i][j] = first and dp[i + 1][j + 1]
    return dp[0][0]


if __name__ == "__main__":
    print(str(is_match("ray", "ra.")).lower())      # true
    print(str(is_match("raymond", "ra.")).lower())  # false
    print(str(is_match("chat", ".*at")).lower())    # true
    print(str(is_match("chats", ".*at")).lower())   # false
