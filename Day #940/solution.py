# Day 940: Time for a message from node 0 to reach all = max shortest-path distance (Dijkstra).
# Time O(E log V), Space O(V + E). Returns -1 if some node is unreachable.
import heapq


def network_delay(n, edges, src):
    adj = [[] for _ in range(n + 1)]
    for a, b, t in edges:
        adj[a].append((b, t))
    dist = [float("inf")] * (n + 1)
    dist[src] = 0
    pq = [(0, src)]
    while pq:
        d, u = heapq.heappop(pq)
        if d > dist[u]:
            continue
        for v, w in adj[u]:
            if d + w < dist[v]:
                dist[v] = d + w
                heapq.heappush(pq, (dist[v], v))
    if any(x == float("inf") for x in dist):
        return -1
    return max(dist)


if __name__ == "__main__":
    n = 5
    edges = [
        (0, 1, 5), (0, 2, 3), (0, 5, 4),
        (1, 3, 8), (2, 3, 1), (3, 5, 10), (3, 4, 5),
    ]
    print(network_delay(n, edges, 0))  # 9
