# Day 209: Longest common subsequence of three strings.
# 3D DP over prefixes: dp[i][j][k]. Time: O(n1*n2*n3), Space: O(n1*n2*n3).


def lcs3(a, b, c):
    n1, n2, n3 = len(a), len(b), len(c)
    dp = [[[0] * (n3 + 1) for _ in range(n2 + 1)] for _ in range(n1 + 1)]
    for i in range(1, n1 + 1):
        for j in range(1, n2 + 1):
            for k in range(1, n3 + 1):
                if a[i - 1] == b[j - 1] == c[k - 1]:
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1
                else:
                    dp[i][j][k] = max(dp[i - 1][j][k], dp[i][j - 1][k], dp[i][j][k - 1])
    return dp[n1][n2][n3]


if __name__ == "__main__":
    print(lcs3("epidemiologist", "refrigeration", "supercalifragilisticexpialodocious"))  # 5
