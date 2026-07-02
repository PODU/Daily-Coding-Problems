# Day 1754: All strobogrammatic numbers with N digits.
# Build from middle outward placing rotatable pairs; skip leading 0 for outer layer.
# Time O(output size), space O(N) recursion depth.

PAIRS = [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')]


def build(n, total):
    if n == 0:
        return [""]
    if n == 1:
        return ["0", "1", "8"]
    inner = build(n - 2, total)
    res = []
    for s in inner:
        for a, b in PAIRS:
            if a == '0' and n == total:  # no leading zero
                continue
            res.append(a + s + b)
    return res


def strobogrammatic(n):
    return build(n, n)


if __name__ == "__main__":
    for n in (2, 3):
        print("N={}: {}".format(n, strobogrammatic(n)))
