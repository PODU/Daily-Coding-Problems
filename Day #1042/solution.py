# Day 1042: LCS of three strings via 3D DP dp[i][j][k]. Time O(n1*n2*n3), Space O(n2*n3) (rolling).

def lcs3(a, b, c):
    n1, n2, n3 = len(a), len(b), len(c)
    prev = [[0] * (n3 + 1) for _ in range(n2 + 1)]
    for i in range(1, n1 + 1):
        cur = [[0] * (n3 + 1) for _ in range(n2 + 1)]
        for j in range(1, n2 + 1):
            for k in range(1, n3 + 1):
                if a[i - 1] == b[j - 1] == c[k - 1]:
                    cur[j][k] = prev[j - 1][k - 1] + 1
                else:
                    cur[j][k] = max(prev[j][k], cur[j - 1][k], cur[j][k - 1])
        prev = cur
    return prev[n2][n3]


if __name__ == "__main__":
    print(lcs3("epidemiologist", "refrigeration",
               "supercalifragilisticexpialodocious"))
