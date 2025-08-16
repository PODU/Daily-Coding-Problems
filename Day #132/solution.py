# Day 132: HitCounter (record, total, range).
# Keep timestamps sorted via bisect; range O(log n), total O(1).
# Limited-memory follow-up: bucket counts by coarse time granularity instead of per-hit.
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
    hc = HitCounter()
    for t in [1, 1, 2, 3, 5, 8, 8, 10]:
        hc.record(t)
    print("total =", hc.total())          # 8
    print("range(2, 8) =", hc.range(2, 8))  # 5
