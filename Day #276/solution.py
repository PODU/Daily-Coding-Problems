# Day 276: KMP pattern search. Time O(N + k), Space O(k) -- beats O(N*k).
# Returns start index of first match, or False.


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
    for text, pat in (("abxabcabcaby", "abcaby"), ("abxabcabcaby", "zzz")):
        r = kmp(text, pat)
        # print -1 for the not-found case so every language reports the same
        print(-1 if r is False else r)  # 6, then -1
