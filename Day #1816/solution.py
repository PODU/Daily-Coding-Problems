# Day 1816: Time-versioned map: per key keep times sorted; get returns value at most-recent time <= t.
# set amortized O(log m), get O(log m) via bisect. Space: O(total entries).
from bisect import bisect_right, insort


class TimeMap:
    def __init__(self):
        self.times = {}   # key -> sorted list of times
        self.values = {}  # key -> {time: value}

    def set(self, key, value, time):
        if key not in self.values:
            self.times[key] = []
            self.values[key] = {}
        if time not in self.values[key]:
            insort(self.times[key], time)
        self.values[key][time] = value  # latest set at same time wins

    def get(self, key, time):
        if key not in self.times:
            return None
        ts = self.times[key]
        i = bisect_right(ts, time)
        if i == 0:
            return None
        return self.values[key][ts[i - 1]]


def show(d, key, time):
    r = d.get(key, time)
    print(f"get({key}, {time}) = {'null' if r is None else r}")


if __name__ == "__main__":
    d = TimeMap(); d.set(1, 1, 0); d.set(1, 2, 2); show(d, 1, 1); show(d, 1, 3)   # 1, 2
    d = TimeMap(); d.set(1, 1, 5); show(d, 1, 0); show(d, 1, 10)                   # null, 1
    d = TimeMap(); d.set(1, 1, 0); d.set(1, 2, 0); show(d, 1, 0)                   # 2
