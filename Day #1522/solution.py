# Day 1522: Concatenation of all equal-length words: sliding window per offset (0..L-1).
# Time O(|s| * L), Space O(words). Each word used exactly once.
from collections import Counter


def find_substring(s, words):
    res = []
    if not words:
        return res
    L = len(words[0])
    k = len(words)
    n = len(s)
    if L * k > n:
        return res
    need = Counter(words)
    for off in range(L):
        left = off
        count = 0
        win = Counter()
        j = off
        while j + L <= n:
            w = s[j:j + L]
            if w in need:
                win[w] += 1
                count += 1
                while win[w] > need[w]:
                    lw = s[left:left + L]
                    win[lw] -= 1
                    left += L
                    count -= 1
                if count == k:
                    res.append(left)
                    lw = s[left:left + L]
                    win[lw] -= 1
                    left += L
                    count -= 1
            else:
                win.clear()
                count = 0
                left = j + L
            j += L
    res.sort()
    return res


if __name__ == "__main__":
    print(find_substring("dogcatcatcodecatdog", ["cat", "dog"]))
    print(find_substring("barfoobazbitbyte", ["dog", "cat"]))
