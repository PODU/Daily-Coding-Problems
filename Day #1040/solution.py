# Day 1040: KMP string matching: build LPS failure array O(k), scan text O(N). Time O(N+k), Space O(k).

def kmp_search(text, pat):
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
    return -1


if __name__ == "__main__":
    text = "abxabcabcaby"
    r1 = kmp_search(text, "abcaby")
    print("Not found" if r1 == -1 else f"Found at index {r1}")
    r2 = kmp_search(text, "xyz")
    print("Not found" if r2 == -1 else f"Found at index {r2}")
