# Day 1613: Skyline via sweep-line + max-heap. Emit key point whenever the current max height changes.
# Time: O(n log n), Space: O(n).
import heapq


def get_skyline(buildings):
    events = []  # (x, signed height): start = -h, end = +h
    for left, right, h in buildings:
        events.append((left, -h))
        events.append((right, h))
    events.sort()

    # max-heap via negatives; lazy deletion with a counter of removed heights
    live = [0]
    removed = {}
    prev = 0
    res = []
    for x, h in events:
        if h < 0:
            heapq.heappush(live, h)          # push -height
        else:
            removed[h] = removed.get(h, 0) + 1  # mark +height for lazy removal
        while live and removed.get(-live[0], 0) > 0:
            removed[-live[0]] -= 1
            heapq.heappop(live)
        cur = -live[0]
        if cur != prev:
            res.append((x, cur))
            prev = cur
    return res


def main():
    buildings = [(0, 15, 3), (4, 11, 5), (19, 23, 4)]
    res = get_skyline(buildings)
    print("[" + ", ".join("({}, {})".format(x, h) for x, h in res) + "]")


if __name__ == "__main__":
    main()
