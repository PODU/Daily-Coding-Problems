# Day 743: Find all bridges with Tarjan's DFS: edge (u,v) is a bridge if low[v] > disc[u].
# Iterative not needed for demo; recursion with sys limit raised for safety.
# Time: O(V + E), Space: O(V + E).
import sys

sys.setrecursionlimit(10**6)


def find_bridges(n, edges):
    adj = [[] for _ in range(n)]
    for u, v in edges:
        adj[u].append(v)
        adj[v].append(u)
    disc = [0] * n
    low = [0] * n
    timer = [0]
    bridges = []

    def dfs(u, parent):
        timer[0] += 1
        disc[u] = low[u] = timer[0]
        skipped_parent = False
        for v in adj[u]:
            if v == parent and not skipped_parent:
                skipped_parent = True
                continue
            if disc[v] == 0:
                dfs(v, u)
                low[u] = min(low[u], low[v])
                if low[v] > disc[u]:
                    bridges.append((min(u, v), max(u, v)))
            else:
                low[u] = min(low[u], disc[v])

    for i in range(n):
        if disc[i] == 0:
            dfs(i, -1)
    return sorted(bridges)


if __name__ == "__main__":
    edges = [(0, 1), (1, 2), (2, 0), (1, 3), (3, 4)]
    for b in find_bridges(5, edges):
        print(b)  # (1, 3) then (3, 4)
