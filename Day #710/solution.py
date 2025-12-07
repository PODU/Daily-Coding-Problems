# Day 710: Find start indices where s contains a concatenation of all equal-length
# words exactly once. Sliding window over wordLen offsets. Time O(n*wordLen).
from collections import Counter


def find_substring(s, words):
    res = []
    if not words:
        return res
    wl, k, n = len(words[0]), len(words), len(s)
    if wl * k > n:
        return res
    need = Counter(words)
    for off in range(wl):
        left, count = off, 0
        window = Counter()
        for j in range(off, n - wl + 1, wl):
            w = s[j:j + wl]
            if w in need:
                window[w] += 1
                count += 1
                while window[w] > need[w]:
                    lw = s[left:left + wl]
                    window[lw] -= 1
                    left += wl
                    count -= 1
                if count == k:
                    res.append(left)
                    lw = s[left:left + wl]
                    window[lw] -= 1
                    left += wl
                    count -= 1
            else:
                window.clear()
                count = 0
                left = j + wl
    return sorted(res)


if __name__ == "__main__":
    print(find_substring("dogcatcatcodecatdog", ["cat", "dog"]))
    print(find_substring("barfoobazbitbyte", ["dog", "cat"]))
