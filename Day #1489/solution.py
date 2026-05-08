# Day 1489: Time-indexed map. Per key, keep entries sorted by time; get does
# binary search for the most recent time <= query. set O(log n), get O(log n).
import bisect


class TimeMap:
    def __init__(self):
        self._times = {}   # key -> sorted list of times
        self._vals = {}    # key -> values aligned with _times

    def set(self, key, value, time):
        ts = self._times.setdefault(key, [])
        vs = self._vals.setdefault(key, [])
        i = bisect.bisect_left(ts, time)
        if i < len(ts) and ts[i] == time:
            vs[i] = value           # same time => later set overwrites
        else:
            ts.insert(i, time)
            vs.insert(i, value)

    def get(self, key, time):
        ts = self._times.get(key)
        if not ts:
            return None
        i = bisect.bisect_right(ts, time) - 1  # rightmost time <= query
        if i < 0:
            return None
        return self._vals[key][i]


def show(d, key, time):
    val = d.get(key, time)
    print("get(%d,%d) = %s" % (key, time, "null" if val is None else val))


if __name__ == "__main__":
    d1 = TimeMap()
    d1.set(1, 1, 0); d1.set(1, 2, 2)
    show(d1, 1, 1)    # 1
    show(d1, 1, 3)    # 2

    d2 = TimeMap()
    d2.set(1, 1, 5)
    show(d2, 1, 0)    # null
    show(d2, 1, 10)   # 1

    d3 = TimeMap()
    d3.set(1, 1, 0); d3.set(1, 2, 0)
    show(d3, 1, 0)    # 2
