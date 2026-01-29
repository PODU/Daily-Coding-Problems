# Day 982: Strobogrammatic numbers of N digits: build recursively from outside in using rotatable
# pairs {0-0,1-1,8-8,6-9,9-6}; skip leading 0 for N>1. Time O(5^(N/2)), Space O(N).

PAIRS = [('0', '0'), ('1', '1'), ('8', '8'), ('6', '9'), ('9', '6')]


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
    print("N=2:", strobogrammatic(2))
    print("N=1:", strobogrammatic(1))
