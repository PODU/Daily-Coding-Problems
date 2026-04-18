# Day 1384: HitCounter: keep timestamps in a sorted list (bisect.insort); record O(n), total O(1), range via binary search (right-left).
import bisect


class HitCounter:
    def __init__(self):
        self.hits = []
        self.cnt = 0

    def record(self, t):
        bisect.insort(self.hits, t)
        self.cnt += 1

    def total(self):
        return self.cnt

    def range(self, lo, hi):
        return bisect.bisect_right(self.hits, hi) - bisect.bisect_left(self.hits, lo)


if __name__ == "__main__":
    hc = HitCounter()
    for t in [1, 1, 2, 3, 5, 8]:
        hc.record(t)
    print(f"total: {hc.total()}")
    print(f"range(2,5): {hc.range(2, 5)}")
