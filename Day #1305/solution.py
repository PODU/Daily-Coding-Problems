# Day 1305: Longest common subsequence of three strings.
# 3D DP over prefixes. O(|a||b||c|) time, O(|a||b||c|) space.


def lcs3(a: str, b: str, c: str) -> int:
    n, m, p = len(a), len(b), len(c)
    dp = [[[0] * (p + 1) for _ in range(m + 1)] for _ in range(n + 1)]
    for i in range(1, n + 1):
        for j in range(1, m + 1):
            for k in range(1, p + 1):
                if a[i - 1] == b[j - 1] == c[k - 1]:
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1
                else:
                    dp[i][j][k] = max(dp[i - 1][j][k], dp[i][j - 1][k], dp[i][j][k - 1])
    return dp[n][m][p]


if __name__ == "__main__":
    print(lcs3("epidemiologist", "refrigeration",
               "supercalifragilisticexpialodocious"))  # 5
