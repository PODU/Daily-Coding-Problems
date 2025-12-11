# Day 734: Time-travel map; get(key,t) returns value set at the most recent time <= t.
# Approach: per key a sorted list of (time, value); get uses binary search (bisect).
# Time: set O(n) insert / O(log n) amortized, get O(log n). Space: O(n).
import bisect


class TimeMap:
    def __init__(self):
        self.store = {}  # key -> dict(time->value) plus sorted times

    def set(self, key, value, time):
        times, vals = self.store.setdefault(key, ([], {}))
        if time not in vals:
            bisect.insort(times, time)
        vals[time] = value

    def get(self, key, time):
        if key not in self.store:
            return None
        times, vals = self.store[key]
        i = bisect.bisect_right(times, time)
        if i == 0:
            return None
        return vals[times[i - 1]]


def show(v):
    print("null" if v is None else v)


if __name__ == "__main__":
    d1 = TimeMap(); d1.set(1, 1, 0); d1.set(1, 2, 2)
    show(d1.get(1, 1))   # 1
    show(d1.get(1, 3))   # 2
    d2 = TimeMap(); d2.set(1, 1, 5)
    show(d2.get(1, 0))   # null
    show(d2.get(1, 10))  # 1
    d3 = TimeMap(); d3.set(1, 1, 0); d3.set(1, 2, 0)
    show(d3.get(1, 0))   # 2
