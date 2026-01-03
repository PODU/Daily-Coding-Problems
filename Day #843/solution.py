# Day 843: find all start indices of pattern in string using KMP.
# Build failure table, single scan. O(n+m) time, O(m) space.

def kmp_search(s, p):
    n, m = len(s), len(p)
    if m == 0 or m > n:
        return []
    lps = [0] * m
    length = 0
    i = 1
    while i < m:
        if p[i] == p[length]:
            length += 1
            lps[i] = length
            i += 1
        elif length:
            length = lps[length - 1]
        else:
            lps[i] = 0
            i += 1
    res = []
    i = j = 0
    while i < n:
        if s[i] == p[j]:
            i += 1
            j += 1
            if j == m:
                res.append(i - m)
                j = lps[j - 1]
        elif j:
            j = lps[j - 1]
        else:
            i += 1
    return res


if __name__ == "__main__":
    print(kmp_search("abracadabra", "abr"))  # [0, 7]
