# Day 929: Shortest uphill-then-downhill cycle from home (node 0).
# Dijkstra on two DAG subgraphs (uphill, reversed downhill), O(E log V) time, O(V+E) space.
import heapq


def dijkstra(adj, src, n):
    dist = [float('inf')] * n
    dist[src] = 0
    pq = [(0, src)]
    while pq:
        d, u = heapq.heappop(pq)
        if d > dist[u]:
            continue
        for v, w in adj[u]:
            if d + w < dist[v]:
                dist[v] = d + w
                heapq.heappush(pq, (d + w, v))
    return dist


def shortest_route(elevations, paths):
    n = max(elevations) + 1
    up = [[] for _ in range(n)]        # uphill edges u->v, elev[v] > elev[u]
    down_rev = [[] for _ in range(n)]  # reversed downhill edges
    for (u, v), w in paths.items():
        if elevations[v] > elevations[u]:
            up[u].append((v, w))
        elif elevations[v] < elevations[u]:
            down_rev[v].append((u, w))  # reverse: downhill u->v becomes v->u
    up_dist = dijkstra(up, 0, n)
    down_dist = dijkstra(down_rev, 0, n)
    best = float('inf')
    for p in range(1, n):  # peak != home
        if up_dist[p] < float('inf') and down_dist[p] < float('inf'):
            best = min(best, up_dist[p] + down_dist[p])
    return best


if __name__ == "__main__":
    elevations = {0: 5, 1: 25, 2: 15, 3: 20, 4: 10}
    paths = {(0, 1): 10, (0, 2): 8, (0, 3): 15, (1, 3): 12,
             (2, 4): 10, (3, 4): 5, (3, 0): 17, (4, 0): 10}
    print(shortest_route(elevations, paths))
