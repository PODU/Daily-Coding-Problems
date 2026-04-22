# Day 1404: KMP: build LPS, scan once, record every full match start. Time O(N+k), Space O(k).

def find_all(s, pat):
    res = []
    N, k = len(s), len(pat)
    if k == 0 or k > N:
        return res
    lps = [0] * k
    length, i = 0, 1
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
                res.append(i - k)
                j = lps[j - 1]
        elif j:
            j = lps[j - 1]
        else:
            i += 1
    return res


if __name__ == "__main__":
    print(find_all("abracadabra", "abr"))  # [0, 7]
