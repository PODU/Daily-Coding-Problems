# Day 1150: Skyline - sweep line over building edges with a max-heap (lazy deletion).
# Track current max height; emit point when it changes. Time O(n log n), Space O(n).
import heapq


def skyline(buildings):
    events = []
    for l, r, h in buildings:
        events.append((l, -h))  # start
        events.append((r, h))   # end
    events.sort()
    heap = [0]
    removed = {}
    prev = 0
    res = []
    for x, h in events:
        if h < 0:
            heapq.heappush(heap, h)  # store as negative for max-heap
        else:
            removed[-h] = removed.get(-h, 0) + 1
        while heap and removed.get(heap[0], 0):
            removed[heap[0]] -= 1
            heapq.heappop(heap)
        cur = -heap[0]
        if cur != prev:
            res.append((x, cur))
            prev = cur
    return res


if __name__ == "__main__":
    bld = [(0, 15, 3), (4, 11, 5), (19, 23, 4)]
    print(skyline(bld))  # [(0, 3), (4, 5), (11, 3), (15, 0), (19, 4), (23, 0)]
