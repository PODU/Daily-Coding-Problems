# Day 262: Find all bridges in a connected undirected graph.
# Approach: Tarjan's bridge-finding algorithm using DFS with disc/low timestamps.
# An edge (u,v) is a bridge iff low[v] > disc[u]. Time O(V+E), Space O(V+E).

import sys
from collections import defaultdict


class BridgeFinder:
    def __init__(self, n):
        self.n = n
        self.adj = defaultdict(list)
        self.timer = 0

    def add_edge(self, u, v):
        self.adj[u].append(v)
        self.adj[v].append(u)

    def find_bridges(self):
        disc = [0] * self.n
        low = [0] * self.n
        bridges = []

        # Iterative DFS to avoid recursion limits on large graphs.
        for start in range(self.n):
            if disc[start] != 0:
                continue
            # stack holds (node, parent, iterator index)
            stack = [(start, -1, 0)]
            self.timer += 1
            disc[start] = low[start] = self.timer
            skipped = {}
            while stack:
                u, parent, idx = stack[-1]
                if idx < len(self.adj[u]):
                    stack[-1] = (u, parent, idx + 1)
                    v = self.adj[u][idx]
                    if v == parent and not skipped.get((u, parent), False):
                        skipped[(u, parent)] = True
                        continue
                    if disc[v] == 0:
                        self.timer += 1
                        disc[v] = low[v] = self.timer
                        stack.append((v, u, 0))
                    else:
                        low[u] = min(low[u], disc[v])
                else:
                    stack.pop()
                    if stack:
                        p = stack[-1][0]
                        low[p] = min(low[p], low[u])
                        if low[u] > disc[p]:
                            bridges.append((min(p, u), max(p, u)))
        bridges.sort()
        return bridges


def main():
    g = BridgeFinder(5)
    for u, v in [(0, 1), (1, 2), (2, 0), (1, 3), (3, 4)]:
        g.add_edge(u, v)
    bridges = g.find_bridges()
    print("Bridges: [" + ", ".join(f"({a}, {b})" for a, b in bridges) + "]")


if __name__ == "__main__":
    main()
