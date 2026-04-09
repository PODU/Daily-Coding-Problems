# Day 1328: Split a string into the fewest palindromic substrings.
# DP: isPal[i][j] in O(n^2); dp[i]=min pieces for prefix i, with parent pointers to rebuild. O(n^2) time/space.

def min_palindrome_partition(s):
    n = len(s)
    if n == 0:
        return []
    pal = [[False] * n for _ in range(n)]
    for i in range(n - 1, -1, -1):
        for j in range(i, n):
            pal[i][j] = s[i] == s[j] and (j - i < 2 or pal[i + 1][j - 1])

    INF = float("inf")
    dp = [INF] * (n + 1)
    prev = [-1] * (n + 1)
    dp[0] = 0
    for i in range(1, n + 1):
        for j in range(i):
            if pal[j][i - 1] and dp[j] + 1 < dp[i]:
                dp[i] = dp[j] + 1
                prev[i] = j

    res = []
    i = n
    while i > 0:
        res.append(s[prev[i]:i])
        i = prev[i]
    return res[::-1]


if __name__ == "__main__":
    print(min_palindrome_partition("racecarannakayak"))  # ['racecar', 'anna', 'kayak']
    print(min_palindrome_partition("abc"))                # ['a', 'b', 'c']
