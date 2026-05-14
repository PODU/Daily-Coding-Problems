# Day 1509: Minimally-connected = tree: edges == n-1 AND connected. Use BFS.
# Time O(n + e), Space O(n).
from collections import deque


def is_tree(n, edges):
    if len(edges) != n - 1:
        return False
    adj = [[] for _ in range(n)]
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)
    visited = [False] * n
    q = deque([0])
    visited[0] = True
    count = 1
    while q:
        u = q.popleft()
        for v in adj[u]:
            if not visited[v]:
                visited[v] = True
                count += 1
                q.append(v)
    return count == n


if __name__ == "__main__":
    n = 4
    edges = [(0, 1), (1, 2), (1, 3)]
    print(is_tree(n, edges))
