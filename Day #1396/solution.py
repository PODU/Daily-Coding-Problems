# Day 1396: KMP substring search: build LPS array, then single scan.
# Time: O(N + k), Space: O(k).

def kmp_search(s, pat):
    N, k = len(s), len(pat)
    if k == 0:
        return 0
    lps = [0] * k
    length = 0
    i = 1
    while i < k:
        if pat[i] == pat[length]:
            length += 1
            lps[i] = length
            i += 1
        elif length:
            length = lps[length - 1]
        else:
            lps[i] = 0
            i += 1
    i = j = 0
    while i < N:
        if s[i] == pat[j]:
            i += 1
            j += 1
            if j == k:
                return i - k
        elif j:
            j = lps[j - 1]
        else:
            i += 1
    return False


if __name__ == "__main__":
    res = kmp_search("abracadabra", "cad")
    print(res if res is not False else "False")
