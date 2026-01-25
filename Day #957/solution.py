# Day 957: skyline problem via sweep line with a max-heap (lazy deletion).
# Time O(n log n), Space O(n).
import heapq
from collections import defaultdict


def skyline(buildings):
    events = []
    for l, r, h in buildings:
        events.append((l, -h))  # start (negative height sorts first)
        events.append((r, h))   # end
    events.sort()
    res = []
    pq = [0]                    # max-heap via negated heights; baseline 0
    removed = defaultdict(int)
    prev = 0
    for x, h in events:
        if h < 0:
            heapq.heappush(pq, h)      # push negated height
        else:
            removed[-h] += 1           # mark negated height for removal
        while pq and removed[pq[0]] > 0:
            removed[pq[0]] -= 1
            heapq.heappop(pq)
        cur = -pq[0]
        if cur != prev:
            res.append((x, cur))
            prev = cur
    return res


if __name__ == "__main__":
    print(skyline([(0, 15, 3), (4, 11, 5), (19, 23, 4)]))
    # [(0, 3), (4, 5), (11, 3), (15, 0), (19, 4), (23, 0)]
