# Day 207: Check if an undirected graph is bipartite.
# BFS 2-coloring: color each component, fail if an edge joins same colors. Handles disconnected graphs.
# Time: O(V + E), Space: O(V).
from collections import deque


def is_bipartite(n, adj):
    color = [-1] * n
    for s in range(n):
        if color[s] != -1:
            continue
        color[s] = 0
        q = deque([s])
        while q:
            u = q.popleft()
            for v in adj[u]:
                if color[v] == -1:
                    color[v] = color[u] ^ 1
                    q.append(v)
                elif color[v] == color[u]:
                    return False
    return True


def graph(edges, n):
    adj = [[] for _ in range(n)]
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)
    return adj


if __name__ == "__main__":
    print(is_bipartite(4, graph([(0, 1), (1, 2), (2, 3), (3, 0)], 4)))  # True
    print(is_bipartite(3, graph([(0, 1), (1, 2), (2, 0)], 3)))          # False
