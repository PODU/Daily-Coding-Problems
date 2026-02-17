# Day 1090: Palindrome pairs: hash words; for each split test palindromic remainder + reversed counterpart.
# Time O(n*k^2), Space O(n*k).
def palindrome_pairs(words):
    d = {w: i for i, w in enumerate(words)}

    def is_pal(s):
        return s == s[::-1]

    res = set()
    for i, word in enumerate(words):
        L = len(word)
        for j in range(L + 1):
            left, right = word[:j], word[j:]
            if is_pal(left):
                r = right[::-1]
                if r in d and d[r] != i:
                    res.add((d[r], i))
            if j != L and is_pal(right):
                l = left[::-1]
                if l in d and d[l] != i:
                    res.add((i, d[l]))
    return sorted(res)


if __name__ == "__main__":
    print(palindrome_pairs(["code", "edoc", "da", "d"]))
