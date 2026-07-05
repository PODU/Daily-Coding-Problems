# Day 1769: Uphill-then-downhill shortest cyclic route from home (node 0): Dijkstra over
# states (node, phase). UP edges require rising elevation, DOWN edges require
# falling; a free phase switch (the peak) is allowed at non-home nodes.
# Time: O(E log V), Space: O(V+E).
import heapq


def shortest_route(elev, paths, home=0):
    adj = {u: [] for u in elev}
    for (u, v), w in paths.items():
        adj[u].append((v, w))

    INF = float("inf")
    # state = (node, phase) phase 0 = up, 1 = down
    dist = {(node, p): INF for node in elev for p in (0, 1)}
    dist[(home, 0)] = 0
    pq = [(0, home, 0)]
    while pq:
        d, node, phase = heapq.heappop(pq)
        if d > dist[(node, phase)]:
            continue
        if phase == 0 and node != home:  # free switch at the peak
            if d < dist[(node, 1)]:
                dist[(node, 1)] = d
                heapq.heappush(pq, (d, node, 1))
        for v, w in adj[node]:
            if phase == 0 and elev[v] > elev[node]:
                if d + w < dist[(v, 0)]:
                    dist[(v, 0)] = d + w
                    heapq.heappush(pq, (d + w, v, 0))
            if phase == 1 and elev[v] < elev[node]:
                if d + w < dist[(v, 1)]:
                    dist[(v, 1)] = d + w
                    heapq.heappush(pq, (d + w, v, 1))
    return dist[(home, 1)]


if __name__ == "__main__":
    elevations = {0: 5, 1: 25, 2: 15, 3: 20, 4: 10}
    paths = {
        (0, 1): 10, (0, 2): 8, (0, 3): 15, (1, 3): 12,
        (2, 4): 10, (3, 4): 5, (3, 0): 17, (4, 0): 10,
    }
    print(shortest_route(elevations, paths))
