# Day 1756: HitCounter design.
# Store timestamps in a sorted list; total() O(1), range() via binary search O(log n).
# Limited-memory follow-up: bucket hits by coarse time granularity (e.g. per-second
# counts in a dict/ring buffer) so memory is O(#buckets) instead of O(#hits).
import bisect


class HitCounter:
    def __init__(self):
        self.hits = []  # kept sorted

    def record(self, timestamp):
        bisect.insort(self.hits, timestamp)

    def total(self):
        return len(self.hits)

    def range(self, lower, upper):
        lo = bisect.bisect_left(self.hits, lower)
        hi = bisect.bisect_right(self.hits, upper)
        return hi - lo


def main():
    hc = HitCounter()
    for t in [1, 2, 2, 5, 7, 9, 10]:
        hc.record(t)

    print("total() =", hc.total())            # 7
    print("range(2, 7) =", hc.range(2, 7))    # 4 (2,2,5,7)
    print("range(0, 10) =", hc.range(0, 10))  # 7


if __name__ == "__main__":
    main()
