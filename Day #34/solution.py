# Day 34: Min-insertion palindrome, lexicographically earliest. DP over substrings; O(n^2) states, O(n^3) overall.
def solve(s):
    n = len(s)
    if n == 0:
        return ""
    dp = [[""] * n for _ in range(n)]
    for i in range(n):
        dp[i][i] = s[i]
    for length in range(2, n + 1):
        for i in range(0, n - length + 1):
            j = i + length - 1
            if s[i] == s[j]:
                inner = dp[i + 1][j - 1] if i + 1 <= j - 1 else ""
                dp[i][j] = s[i] + inner + s[j]
            else:
                c1 = s[i] + dp[i + 1][j] + s[i]
                c2 = s[j] + dp[i][j - 1] + s[j]
                if len(c1) < len(c2):
                    dp[i][j] = c1
                elif len(c2) < len(c1):
                    dp[i][j] = c2
                else:
                    dp[i][j] = min(c1, c2)
    return dp[0][n - 1]


if __name__ == "__main__":
    print('"' + solve("race") + '"')
