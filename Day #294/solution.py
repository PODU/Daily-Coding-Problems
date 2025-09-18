# Day 294: Uphill-then-downhill closed route: Dijkstra on up-edges from 0, Dijkstra on reversed down-edges
# to 0; answer = min over peaks of distUp[m]+distDown[m]. Time O((V+E)logV), Space O(V+E).
import heapq


def dijkstra(n, adj, src):
    d = [float("inf")] * n
    d[src] = 0
    pq = [(0, src)]
    while pq:
        du, u = heapq.heappop(pq)
        if du > d[u]:
            continue
        for v, w in adj[u]:
            if d[u] + w < d[v]:
                d[v] = d[u] + w
                heapq.heappush(pq, (d[v], v))
    return d


if __name__ == "__main__":
    n = 5
    elev = {0: 5, 1: 25, 2: 15, 3: 20, 4: 10}
    paths = {(0, 1): 10, (0, 2): 8, (0, 3): 15, (1, 3): 12,
             (2, 4): 10, (3, 4): 5, (3, 0): 17, (4, 0): 10}
    up = [[] for _ in range(n)]
    down_rev = [[] for _ in range(n)]
    for (u, v), w in paths.items():
        if elev[v] > elev[u]:
            up[u].append((v, w))
        if elev[v] < elev[u]:
            down_rev[v].append((u, w))  # reversed for m->0 search
    dist_up = dijkstra(n, up, 0)
    dist_down = dijkstra(n, down_rev, 0)
    best = float("inf")
    for m in range(1, n):
        if dist_up[m] < float("inf") and dist_down[m] < float("inf"):
            best = min(best, dist_up[m] + dist_down[m])
    print(best)
