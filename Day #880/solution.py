# Day 880: Min palindrome partition via DP with palindrome table + reconstruction.
# Time O(n^2), Space O(n^2).

def min_palindrome_partition(s):
    n = len(s)
    if n == 0:
        return []
    pal = [[False] * n for _ in range(n)]
    for i in range(n - 1, -1, -1):
        for j in range(i, n):
            if s[i] == s[j] and (j - i < 2 or pal[i + 1][j - 1]):
                pal[i][j] = True

    INF = float("inf")
    dp = [INF] * (n + 1)
    cut = [-1] * (n + 1)
    dp[0] = 0
    for i in range(1, n + 1):
        for j in range(i):
            if pal[j][i - 1] and dp[j] + 1 < dp[i]:
                dp[i] = dp[j] + 1
                cut[i] = j

    res = []
    i = n
    while i > 0:
        res.append(s[cut[i]:i])
        i = cut[i]
    return res[::-1]


if __name__ == "__main__":
    print(min_palindrome_partition("racecarannakayak"))
    print(min_palindrome_partition("abc"))
