# Day 1848: Longest common subsequence of three strings via 3D dynamic programming.
# Time O(L1*L2*L3), Space O(L1*L2*L3).


def lcs3(a, b, c):
    la, lb, lc = len(a), len(b), len(c)
    dp = [[[0] * (lc + 1) for _ in range(lb + 1)] for _ in range(la + 1)]
    for i in range(1, la + 1):
        for j in range(1, lb + 1):
            for k in range(1, lc + 1):
                if a[i - 1] == b[j - 1] == c[k - 1]:
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1
                else:
                    dp[i][j][k] = max(dp[i - 1][j][k], dp[i][j - 1][k], dp[i][j][k - 1])
    return dp[la][lb][lc]


def main():
    print(lcs3("epidemiologist", "refrigeration",
               "supercalifragilisticexpialodocious"))  # 5


if __name__ == "__main__":
    main()
