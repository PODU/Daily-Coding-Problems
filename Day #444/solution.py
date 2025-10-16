# Day 444: KMP string matching in O(N + k) time, O(k) space (beats O(N*k)).
# Returns the start index of the first match, or False if absent.


def build_lps(p):
    lps = [0] * len(p)
    length = 0
    i = 1
    while i < len(p):
        if p[i] == p[length]:
            length += 1
            lps[i] = length
            i += 1
        elif length:
            length = lps[length - 1]
        else:
            lps[i] = 0
            i += 1
    return lps


def kmp_search(text, pat):
    if not pat:
        return 0
    lps = build_lps(pat)
    i = j = 0
    while i < len(text):
        if text[i] == pat[j]:
            i += 1
            j += 1
            if j == len(pat):
                return i - j
        elif j:
            j = lps[j - 1]
        else:
            i += 1
    return False


if __name__ == "__main__":
    idx = kmp_search("abxabcabcaby", "abcaby")
    print(idx)  # 6
