# Day 1504: KMP string matching. Returns first occurrence index, or False if not found.
# Time O(N+k), Space O(k).

def kmp(text, pat):
    n, k = len(text), len(pat)
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
    while i < n:
        if text[i] == pat[j]:
            i += 1
            j += 1
            if j == k:
                return i - j
        elif j:
            j = lps[j - 1]
        else:
            i += 1
    return False


if __name__ == "__main__":
    print(kmp("abxabcabcaby", "abcaby"))
