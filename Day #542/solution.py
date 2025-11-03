# Day 542: Bipartite check via BFS two-coloring. O(V+E) time, O(V) space.
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


def build(n, edges):
    adj = [[] for _ in range(n)]
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)
    return adj


if __name__ == "__main__":
    print("true" if is_bipartite(4, build(4, [(0, 1), (1, 2), (2, 3), (3, 0)])) else "false")
    print("true" if is_bipartite(3, build(3, [(0, 1), (1, 2), (2, 0)])) else "false")
