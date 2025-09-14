# Day 270: Broadcast time = max shortest-path distance from node 0 (Dijkstra).
# Min-heap Dijkstra over undirected weighted graph; answer = max finite dist. Time O(E log V), Space O(V+E).
import heapq


def network_delay(n, edges):
    adj = [[] for _ in range(n + 1)]
    for a, b, t in edges:
        adj[a].append((b, t))
        adj[b].append((a, t))
    dist = [float('inf')] * (n + 1)
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
    edges = [
        (0, 1, 5), (0, 2, 3), (0, 5, 4),
        (1, 3, 8), (2, 3, 1), (3, 5, 10), (3, 4, 5),
    ]
    print(network_delay(5, edges))
