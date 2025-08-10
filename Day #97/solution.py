# Day 97: Time-versioned map. Each key keeps (time -> value) sorted by time; get
# binary-searches the latest time <= query. set/get O(log n) (amortized).
import bisect


class TimeMap:
    def __init__(self):
        self.store = {}  # key -> ([times], [values])

    def set(self, key, value, time):
        times, vals = self.store.setdefault(key, ([], []))
        i = bisect.bisect_left(times, time)
        if i < len(times) and times[i] == time:
            vals[i] = value  # overwrite same timestamp
        else:
            times.insert(i, time)
            vals.insert(i, value)

    def get(self, key, time):
        if key not in self.store:
            return None
        times, vals = self.store[key]
        i = bisect.bisect_right(times, time) - 1  # latest time <= query
        return vals[i] if i >= 0 else None


def _p(v):
    print(v if v is not None else "null")


if __name__ == "__main__":
    # The README's three blocks are independent scenarios (fresh maps).
    a = TimeMap()
    a.set(1, 1, 0); a.set(1, 2, 2)
    _p(a.get(1, 1))   # 1
    _p(a.get(1, 3))   # 2

    b = TimeMap()
    b.set(1, 1, 5)
    _p(b.get(1, 0))   # null
    _p(b.get(1, 10))  # 1

    c = TimeMap()
    c.set(1, 1, 0); c.set(1, 2, 0)
    _p(c.get(1, 0))   # 2
