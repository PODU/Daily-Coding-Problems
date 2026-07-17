# Day 1826: Dijkstra from node 0; answer is the max shortest-path distance (broadcast time).
# O((V+E) log V).
import heapq


def broadcast_time(edges):
    max_node = max(max(a, b) for a, b, _ in edges)
    V = max_node + 1
    adj = [[] for _ in range(V)]
    for a, b, t in edges:
        adj[a].append((b, t))
        adj[b].append((a, t))

    dist = [float("inf")] * V
    dist[0] = 0
    pq = [(0, 0)]
    while pq:
        d, u = heapq.heappop(pq)
        if d > dist[u]:
            continue
        for v, w in adj[u]:
            if d + w < dist[v]:
                dist[v] = d + w
                heapq.heappush(pq, (dist[v], v))
    return max(dist)


if __name__ == "__main__":
    edges = [(0, 1, 5), (0, 2, 3), (0, 5, 4), (1, 3, 8), (2, 3, 1), (3, 5, 10), (3, 4, 5)]
    print(broadcast_time(edges))  # 9
