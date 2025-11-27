# Day 663: HitCounter. Keep timestamps sorted; total = O(1); range = binary search.
# record O(log n) insert, total O(1), range O(log n).
# Limited-memory follow-up: bucket hits into fixed time windows storing (window, count).
import bisect


class HitCounter:
    def __init__(self):
        self.ts = []

    def record(self, t):
        bisect.insort(self.ts, t)

    def total(self):
        return len(self.ts)

    def range(self, lo, hi):
        return bisect.bisect_right(self.ts, hi) - bisect.bisect_left(self.ts, lo)


if __name__ == "__main__":
    h = HitCounter()
    for t in [1, 2, 2, 5, 9, 10]:
        h.record(t)
    print("total:", h.total())          # 6
    print("range(2,9):", h.range(2, 9))  # 4
