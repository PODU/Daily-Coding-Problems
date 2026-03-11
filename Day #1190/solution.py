# Day 1190: Dijkstra from node 0 over nodes 0..N (undirected); answer = max finite shortest-path distance.
# Time: O(E log V), Space: O(V + E).
import heapq


def network_delay(N, edges):
    V = N + 1
    adj = [[] for _ in range(V)]
    for a, b, t in edges:
        adj[a].append((b, t))
        adj[b].append((a, t))
    dist = [float('inf')] * V
    dist[0] = 0
    pq = [(0, 0)]
    while pq:
        d, u = heapq.heappop(pq)
        if d > dist[u]:
            continue
        for v, w in adj[u]:
            if dist[u] + w < dist[v]:
                dist[v] = dist[u] + w
                heapq.heappush(pq, (dist[v], v))
    return max(d for d in dist if d != float('inf'))


if __name__ == "__main__":
    N = 5
    edges = [(0, 1, 5), (0, 2, 3), (0, 5, 4), (1, 3, 8), (2, 3, 1), (3, 5, 10), (3, 4, 5)]
    print(network_delay(N, edges))
