# Day 831: Find all start indices of substrings that are a concatenation of every word once.
# Sliding window per offset 0..L-1 with hashmap word counts. O(n * L) total ~ O(n).


def find_substring(s, words):
    if not words or not s:
        return []
    L = len(words[0])
    m = len(words)
    window = L * m
    n = len(s)
    if window > n:
        return []
    need = {}
    for w in words:
        need[w] = need.get(w, 0) + 1

    res = []
    for offset in range(L):
        left = offset
        count = 0
        have = {}
        right = offset
        while right + L <= n:
            w = s[right:right + L]
            right += L
            if w in need:
                have[w] = have.get(w, 0) + 1
                count += 1
                while have[w] > need[w]:
                    lw = s[left:left + L]
                    have[lw] -= 1
                    left += L
                    count -= 1
                if count == m:
                    res.append(left)
                    lw = s[left:left + L]
                    have[lw] -= 1
                    left += L
                    count -= 1
            else:
                have.clear()
                count = 0
                left = right
    res.sort()
    return res


def main():
    print(find_substring("dogcatcatcodecatdog", ["cat", "dog"]))  # [0, 13]
    print(find_substring("barfoobazbitbyte", ["dog", "cat"]))     # []


if __name__ == "__main__":
    main()
