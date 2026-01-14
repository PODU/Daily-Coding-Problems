# Day 894: HitCounter: maintain timestamps in a sorted list using bisect. record = insort O(n),
# total = len O(1), range = bisect_right - bisect_left via binary search O(log n).
# Limited-memory follow-up: bucket hits by time window (circular buffer of fixed size)
# so memory stays O(window) instead of O(#hits), trading exact old-range queries for recency.
import bisect


class HitCounter:
    def __init__(self):
        self.ts = []

    def record(self, t):
        bisect.insort(self.ts, t)

    def total(self):
        return len(self.ts)

    def range(self, lower, upper):
        return bisect.bisect_right(self.ts, upper) - bisect.bisect_left(self.ts, lower)


def main():
    hc = HitCounter()
    hc.record(1)
    hc.record(2)
    hc.record(3)
    hc.record(2)
    print(hc.total())       # 4
    print(hc.range(2, 3))   # 3


if __name__ == "__main__":
    main()
