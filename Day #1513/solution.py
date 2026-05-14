# Day 1513: KMP pattern matching: build failure (LPS) array, then scan once collecting all match starts.
# Time: O(N+k), Space: O(k).
def find_all(s, p):
    n, k = len(s), len(p)
    if k == 0 or k > n:
        return []
    lps = [0] * k
    length = 0
    i = 1
    while i < k:
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
            if j == k:
                res.append(i - k)
                j = lps[j - 1]
        elif j:
            j = lps[j - 1]
        else:
            i += 1
    return res


if __name__ == "__main__":
    print(find_all("abracadabra", "abr"))
