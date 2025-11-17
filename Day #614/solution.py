# Day 614: Time for a message from node 0 to reach all nodes = max shortest-path distance.
# Approach: Dijkstra from node 0, return the largest distance. Time O(E log V), Space O(V+E).
import heapq


def broadcast_time(n, edges):
    adj = [[] for _ in range(n + 1)]
    for a, b, t in edges:
        adj[a].append((b, t))
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


def main():
    N = 5
    edges = [
        (0, 1, 5), (0, 2, 3), (0, 5, 4), (1, 3, 8),
        (2, 3, 1), (3, 5, 10), (3, 4, 5),
    ]
    print(broadcast_time(N, edges))  # 9


if __name__ == '__main__':
    main()
