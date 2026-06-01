# Day 1595: Approach: DP min palindrome partition. pal[i][j] table O(n^2), cut[i]=min cuts for prefix,
# then reconstruct one optimal partition. Time O(n^2), Space O(n^2).

def min_pal_partition(s):
    n = len(s)
    if n == 0:
        return []
    pal = [[False] * n for _ in range(n)]
    for i in range(n):
        pal[i][i] = True
    for length in range(2, n + 1):
        for i in range(0, n - length + 1):
            j = i + length - 1
            if s[i] == s[j] and (length == 2 or pal[i + 1][j - 1]):
                pal[i][j] = True
    cut = [0] * n
    start = [0] * n
    for i in range(n):
        if pal[0][i]:
            cut[i] = 0
            start[i] = 0
            continue
        best, bj = float('inf'), 0
        for j in range(1, i + 1):
            if pal[j][i] and cut[j - 1] + 1 < best:
                best = cut[j - 1] + 1
                bj = j
        cut[i] = best
        start[i] = bj
    res = []
    i = n - 1
    while i >= 0:
        j = start[i]
        res.append(s[j:i + 1])
        i = j - 1
    res.reverse()
    return res

def fmt(v):
    return "[" + ", ".join('"' + x + '"' for x in v) + "]"

if __name__ == "__main__":
    print(fmt(min_pal_partition("racecarannakayak")))
    print(fmt(min_pal_partition("abc")))
