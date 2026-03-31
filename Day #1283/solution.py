# Day 1283: For each element, count smaller elements to its right.
# Fenwick (BIT) over compressed values, scanning right-to-left. Time O(n log n), Space O(n).
import bisect


def count_smaller(a):
    n = len(a)
    sorted_vals = sorted(set(a))
    tree = [0] * (len(sorted_vals) + 1)

    def update(i):
        while i < len(tree):
            tree[i] += 1
            i += i & -i

    def query(i):
        s = 0
        while i > 0:
            s += tree[i]
            i -= i & -i
        return s

    res = [0] * n
    for i in range(n - 1, -1, -1):
        rank = bisect.bisect_left(sorted_vals, a[i]) + 1
        res[i] = query(rank - 1)
        update(rank)
    return res


if __name__ == "__main__":
    print(count_smaller([3, 4, 9, 6, 1]))  # [1, 1, 2, 1, 0]
