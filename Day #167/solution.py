# Day 167: Palindrome pairs: hash map of reversed words; for each word check prefix/suffix palindrome splits. O(n*k^2) time, O(n*k) space.

def is_palin(s, i, j):
    while i < j:
        if s[i] != s[j]:
            return False
        i += 1
        j -= 1
    return True


def palindrome_pairs(words):
    rev = {w[::-1]: i for i, w in enumerate(words)}
    res = set()
    for i, w in enumerate(words):
        n = len(w)
        for cut in range(n + 1):
            # left part palindrome -> reversed suffix word goes before w
            if is_palin(w, 0, cut - 1):
                suf = w[cut:]
                if suf in rev and rev[suf] != i:
                    res.add((rev[suf], i))
            # right part palindrome -> reversed prefix word goes after w
            if cut < n and is_palin(w, cut, n - 1):
                pre = w[:cut]
                if pre in rev and rev[pre] != i:
                    res.add((i, rev[pre]))
    return sorted(res)


def main():
    words = ["code", "edoc", "da", "d"]
    pairs = palindrome_pairs(words)
    print("[" + ", ".join("({}, {})".format(a, b) for a, b in pairs) + "]")


if __name__ == "__main__":
    main()
