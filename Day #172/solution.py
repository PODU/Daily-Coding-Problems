# Day 172: Substring concatenation of all words: sliding window over wordLen offsets with hash-map counts.
# Time O(n * wordLen), Space O(words * wordLen).
from collections import Counter


def find_substring(s, words):
    if not words or not s:
        return []
    wl, nw = len(words[0]), len(words)
    total, n = wl * nw, len(s)
    if total > n:
        return []
    need = Counter(words)
    res = []
    for i in range(wl):
        left, count = i, 0
        window = Counter()
        j = i
        while j + wl <= n:
            w = s[j:j + wl]
            if w in need:
                window[w] += 1
                count += 1
                while window[w] > need[w]:
                    lw = s[left:left + wl]
                    window[lw] -= 1
                    count -= 1
                    left += wl
                if count == nw:
                    res.append(left)
                    lw = s[left:left + wl]
                    window[lw] -= 1
                    count -= 1
                    left += wl
            else:
                window.clear()
                count = 0
                left = j + wl
            j += wl
    return sorted(res)


if __name__ == "__main__":
    print(find_substring("dogcatcatcodecatdog", ["cat", "dog"]))
    print(find_substring("barfoobazbitbyte", ["dog", "cat"]))
