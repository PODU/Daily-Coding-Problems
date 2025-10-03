# Day 362: Strobogrammatic numbers of N digits.
# Recursively build from outside in using rotatable digit pairs; skip leading 0.
# Time O(output size), Space O(N) recursion depth.
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
            if n == total and a == '0':
                continue
            res.append(a + s + b)
    return res


def strobogrammatic(n):
    return build(n, n)


if __name__ == "__main__":
    n = 2
    print(f"N={n}: " + " ".join(strobogrammatic(n)))
