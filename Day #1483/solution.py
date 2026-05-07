# Day 1483: Shortest closed route from home (0) that strictly ascends then
# strictly descends. up[v] = shortest strictly-ascending path 0->v (Dijkstra on
# edges with increasing elevation); down[v] = shortest strictly-descending path
# v->0 (Dijkstra from 0 on the reversed descending graph). Answer = min over
# peaks v of up[v]+down[v]. Time O((V+E) log V), Space O(V+E).
import heapq


def dijkstra(n, adj, src):
    dist = [float("inf")] * n
    pred = [-1] * n
    dist[src] = 0
    pq = [(0, src)]
    while pq:
        d, u = heapq.heappop(pq)
        if d > dist[u]:
            continue
        for v, w in adj[u]:
            if d + w < dist[v]:
                dist[v] = d + w
                pred[v] = u
                heapq.heappush(pq, (dist[v], v))
    return dist, pred


def shortest_route(elevations, paths):
    n = len(elevations)
    up_adj = [[] for _ in range(n)]    # ascending edges a->b
    rev_desc = [[] for _ in range(n)]  # reversed descending edges b->a
    for (a, b), w in paths.items():
        if elevations[b] > elevations[a]:
            up_adj[a].append((b, w))
        elif elevations[b] < elevations[a]:
            rev_desc[b].append((a, w))

    up, up_pred = dijkstra(n, up_adj, 0)
    down, down_pred = dijkstra(n, rev_desc, 0)

    best, peak = float("inf"), -1
    for v in range(1, n):
        if up[v] > 0 and down[v] > 0 and up[v] + down[v] < best:
            best, peak = up[v] + down[v], v
    if peak == -1:
        return None, []

    # up path 0 -> peak
    up_path = []
    cur = peak
    while cur != -1:
        up_path.append(cur)
        cur = up_pred[cur]
    up_path.reverse()
    # down path peak -> 0 (following down_pred yields it directly)
    down_path = []
    cur = peak
    while cur != -1:
        down_path.append(cur)
        cur = down_pred[cur]
    route = up_path + down_path[1:]
    return best, route


if __name__ == "__main__":
    elevations = {0: 5, 1: 25, 2: 15, 3: 20, 4: 10}
    paths = {
        (0, 1): 10, (0, 2): 8, (0, 3): 15, (1, 3): 12,
        (2, 4): 10, (3, 4): 5, (3, 0): 17, (4, 0): 10,
    }
    dist, route = shortest_route(elevations, paths)
    print(f"The shortest valid path would be {' -> '.join(map(str, route))}, "
          f"with a distance of {dist}.")
    # The shortest valid path would be 0 -> 2 -> 4 -> 0, with a distance of 28.
