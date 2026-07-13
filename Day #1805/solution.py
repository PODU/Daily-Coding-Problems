# Day 1805: Find all bridges via Tarjan's algorithm (disc/low, edge is bridge if low[v] > disc[u]).
# Parallel edges handled by skipping the parent edge once via edge id. O(V+E).
import sys


def find_bridges(n, edges):
    adj = [[] for _ in range(n)]
    for i, (u, v) in enumerate(edges):
        adj[u].append((v, i))
        adj[v].append((u, i))

    disc = [-1] * n
    low = [-1] * n
    timer = [0]
    result = []

    def dfs(u, parent_edge):
        disc[u] = low[u] = timer[0]
        timer[0] += 1
        for v, eid in adj[u]:
            if eid == parent_edge:          # skip the single parent edge once
                continue
            if disc[v] == -1:
                dfs(v, eid)
                low[u] = min(low[u], low[v])
                if low[v] > disc[u]:
                    result.append((min(u, v), max(u, v)))
            else:
                low[u] = min(low[u], disc[v])

    for i in range(n):
        if disc[i] == -1:
            dfs(i, -1)
    return sorted(result)


def main():
    sys.setrecursionlimit(10000)
    n = 5
    edges = [(0, 1), (1, 2), (2, 0), (1, 3), (3, 4)]
    for u, v in find_bridges(n, edges):
        print(f"{u} - {v}")
    # expected: 1 - 3 and 3 - 4


if __name__ == "__main__":
    main()
