# Day 1802: Palindrome pairs: map word->index, split each word, match palindromic halves.
# Time O(N*L^2), Space O(N*L).

def palindrome_pairs(words):
    idx = {w: i for i, w in enumerate(words)}
    res = set()
    for i, w in enumerate(words):
        n = len(w)
        for j in range(n + 1):
            prefix, suffix = w[:j], w[j:]
            # case 1: prefix palindrome -> reverse(suffix) + w
            if prefix == prefix[::-1]:
                rs = suffix[::-1]
                if rs in idx and idx[rs] != i:
                    res.add((idx[rs], i))
            # case 2: suffix palindrome (non-empty) -> w + reverse(prefix)
            if suffix and suffix == suffix[::-1]:
                rp = prefix[::-1]
                if rp in idx and idx[rp] != i:
                    res.add((i, idx[rp]))
    return sorted(res)


if __name__ == "__main__":
    words = ["code", "edoc", "da", "d"]
    print(palindrome_pairs(words))
