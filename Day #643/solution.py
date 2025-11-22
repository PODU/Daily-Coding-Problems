# Day 643: Longest common subsequence of three strings.
# Approach: 3D DP over prefix lengths of a, b, c.
# Time: O(|a|*|b|*|c|), Space: O(|b|*|c|) (two rolling layers).
def lcs3(a, b, c):
    B, C = len(b), len(c)
    prev = [[0] * (C + 1) for _ in range(B + 1)]
    for i in range(1, len(a) + 1):
        cur = [[0] * (C + 1) for _ in range(B + 1)]
        for j in range(1, B + 1):
            for k in range(1, C + 1):
                if a[i-1] == b[j-1] == c[k-1]:
                    cur[j][k] = prev[j-1][k-1] + 1
                else:
                    cur[j][k] = max(prev[j][k], cur[j-1][k], cur[j][k-1])
        prev = cur
    return prev[B][C]


if __name__ == "__main__":
    print(lcs3("epidemiologist", "refrigeration",
               "supercalifragilisticexpialodocious"))  # 5
