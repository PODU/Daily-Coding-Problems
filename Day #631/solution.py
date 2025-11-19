# Day 631: Skyline problem.
# Approach: sweep line over edges + max-heap of active heights (lazy deletion).
# Time: O(n log n), Space: O(n).
import heapq


def get_skyline(buildings):
    events = []
    for left, right, h in buildings:
        events.append((left, -h))   # start
        events.append((right, h))   # end
    events.sort()

    live = [0]          # max-heap via negatives
    cnt = {0: 1}        # count of each live height

    def push(x):
        heapq.heappush(live, -x)
        cnt[x] = cnt.get(x, 0) + 1

    def remove(x):
        cnt[x] -= 1

    def top():
        while cnt.get(-live[0], 0) == 0:
            heapq.heappop(live)
        return -live[0]

    res = []
    prev = 0
    for x, h in events:
        if h < 0:
            push(-h)
        else:
            remove(h)
        cur = top()
        if cur != prev:
            res.append((x, cur))
            prev = cur
    return res


if __name__ == "__main__":
    buildings = [(0, 15, 3), (4, 11, 5), (19, 23, 4)]
    print(get_skyline(buildings))
