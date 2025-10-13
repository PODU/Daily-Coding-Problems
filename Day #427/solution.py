# Day 427: Shortest uphill-then-downhill route from/to home (location 0).
# State Dijkstra: each node split into (up) and (down) phases; switch at the peak.
# Up edges need strictly higher elevation, down edges strictly lower. Time O((V+E)logV).
import heapq


def solve(elev, paths):
    adj = {}
    for (u, v), d in paths.items():
        adj.setdefault(u, []).append((v, d))
    home = 0
    INF = float('inf')
    # state = (node, phase); phase 0 = uphill, 1 = downhill
    dist = {(home, 0): 0}
    prev = {(home, 0): None}
    pq = [(0, home, 0)]
    goal = (home, 1)
    while pq:
        d, u, ph = heapq.heappop(pq)
        if d > dist.get((u, ph), INF):
            continue
        # switch up -> down at the peak (cannot be home; must have climbed)
        if ph == 0 and u != home:
            ns = (u, 1)
            if d < dist.get(ns, INF):
                dist[ns] = d
                prev[ns] = (u, ph)
                heapq.heappush(pq, (d, u, 1))
        for v, w in adj.get(u, []):
            if ph == 0 and elev[v] > elev[u]:
                ns = (v, 0)
            elif ph == 1 and elev[v] < elev[u]:
                ns = (v, 1)
            else:
                continue
            nd = d + w
            if nd < dist.get(ns, INF):
                dist[ns] = nd
                prev[ns] = (u, ph)
                heapq.heappush(pq, (nd, ns[0], ns[1]))
    if goal not in dist:
        print("No valid route")
        return
    # reconstruct, collapsing the duplicate node created by the up->down switch
    nodes = []
    cur = goal
    while cur is not None:
        nodes.append(cur[0])
        cur = prev[cur]
    nodes.reverse()
    path = []
    for n in nodes:
        if not path or path[-1] != n:
            path.append(n)
    print(" -> ".join(map(str, path)) + f", distance {dist[goal]}")


if __name__ == "__main__":
    elevations = {0: 5, 1: 25, 2: 15, 3: 20, 4: 10}
    paths = {(0, 1): 10, (0, 2): 8, (0, 3): 15, (1, 3): 12,
             (2, 4): 10, (3, 4): 5, (3, 0): 17, (4, 0): 10}
    solve(elevations, paths)
