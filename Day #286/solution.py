# Day 286: Skyline problem.
# Sweep line over events with a max-heap of active (height, end).
# Time: O(n log n), Space: O(n).
import heapq


def get_skyline(buildings):
    events = []
    for l, r, h in buildings:
        events.append((l, -h, r))
        events.append((r, 0, 0))
    events.sort()
    live = [(0, float('inf'))]  # (-height, end)
    res = []
    for x, negh, r in events:
        while live[0][1] <= x:
            heapq.heappop(live)
        if negh:
            heapq.heappush(live, (negh, r))
        cur = -live[0][0]
        if not res or res[-1][1] != cur:
            res.append((x, cur))
    return res


if __name__ == "__main__":
    buildings = [(0, 15, 3), (4, 11, 5), (19, 23, 4)]
    print(get_skyline(buildings))
