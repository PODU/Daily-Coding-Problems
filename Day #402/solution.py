# Day 402: Strobogrammatic numbers of N digits: recursive build outside-in, skip leading 0 pair.
# Time O(5^(N/2)) results, Space O(N) recursion depth.
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
            if n == total and a == '0':  # no leading zero
                continue
            res.append(a + s + b)
    return res


def main():
    res = build(2, 2)
    print("[" + ", ".join('"' + x + '"' for x in res) + "]")


if __name__ == "__main__":
    main()
