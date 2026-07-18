# Day 1838: Count smaller elements to the right, via a Fenwick tree over compressed values.
# Time O(N log N), Space O(N).
import bisect


class BIT:
    def __init__(self, n):
        self.t = [0] * (n + 1)

    def add(self, i):
        while i < len(self.t):
            self.t[i] += 1
            i += i & -i

    def sum(self, i):
        s = 0
        while i > 0:
            s += self.t[i]
            i -= i & -i
        return s


def count_smaller(a):
    uniq = sorted(set(a))
    bit = BIT(len(uniq))
    res = [0] * len(a)
    for i in range(len(a) - 1, -1, -1):
        r = bisect.bisect_left(uniq, a[i]) + 1  # 1-indexed rank
        res[i] = bit.sum(r - 1)
        bit.add(r)
    return res


def main():
    print(count_smaller([3, 4, 9, 6, 1]))


if __name__ == "__main__":
    main()
