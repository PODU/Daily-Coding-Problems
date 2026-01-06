# Day 862: Find all bridges in a connected undirected graph.
# Approach: Tarjan's DFS using discovery times and low-link values (iterative-safe recursion).
# Time: O(V + E), Space: O(V + E).
import sys


def find_bridges(n, edges):
    adj = [[] for _ in range(n)]
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)
    disc = [0] * n
    low = [0] * n
    timer = [0]
    bridges = []

    def dfs(u, parent):
        timer[0] += 1
        disc[u] = low[u] = timer[0]
        for v in adj[u]:
            if v == parent:
                continue
            if disc[v]:
                low[u] = min(low[u], disc[v])
            else:
                dfs(v, u)
                low[u] = min(low[u], low[v])
                if low[v] > disc[u]:
                    bridges.append((min(u, v), max(u, v)))

    for i in range(n):
        if not disc[i]:
            dfs(i, -1)
    return sorted(bridges)


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    res = find_bridges(5, [(0, 1), (1, 2), (2, 0), (1, 3), (3, 4)])
    print("Bridges:", " ".join(f"({a}, {b})" for a, b in res))  # (1, 3) (3, 4)
