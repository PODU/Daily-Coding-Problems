# Day 1047: Time-versioned map: per key a sorted list of (time,value); get uses bisect for floor.
# set/get O(log n) lookup. Setting same key+time overwrites.
import bisect


class TimeMap:
    def __init__(self):
        self.store = {}  # key -> (times_list, values_list)

    def set(self, key, value, time):
        if key not in self.store:
            self.store[key] = ([], [])
        times, values = self.store[key]
        idx = bisect.bisect_left(times, time)
        if idx < len(times) and times[idx] == time:
            values[idx] = value  # overwrite same time
        else:
            times.insert(idx, time)
            values.insert(idx, value)

    def get(self, key, time):
        if key not in self.store:
            return None
        times, values = self.store[key]
        idx = bisect.bisect_right(times, time)
        if idx == 0:
            return None
        return values[idx - 1]


def show(v):
    return "null" if v is None else str(v)


if __name__ == "__main__":
    # README presents three logical blocks; each starts from its own map state.
    d = TimeMap()
    d.set(1, 1, 0)
    d.set(1, 2, 2)
    print("d.get(1, 1) ->", show(d.get(1, 1)))
    print("d.get(1, 3) ->", show(d.get(1, 3)))

    d = TimeMap()
    d.set(1, 1, 5)
    print("d.get(1, 0) ->", show(d.get(1, 0)))
    print("d.get(1, 10) ->", show(d.get(1, 10)))

    d = TimeMap()
    d.set(1, 1, 0)
    d.set(1, 2, 0)  # overwrite same key+time -> value 2
    print("d.get(1, 0) ->", show(d.get(1, 0)))
