# Day 686: Remove max edges so each resulting subtree has an even node count.
# DFS subtree sizes; count non-root nodes whose subtree size is even (each = one removable edge above it). O(n) time/space.
import sys
from collections import defaultdict


def solve(n, edges, root=1):
    adj = defaultdict(list)
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)

    removable = 0

    def dfs(u, parent):
        nonlocal removable
        size = 1
        for v in adj[u]:
            if v != parent:
                size += dfs(v, u)
        if parent != -1 and size % 2 == 0:
            removable += 1
        return size

    sys.setrecursionlimit(10000)
    dfs(root, -1)
    return removable


if __name__ == '__main__':
    edges = [(1, 2), (1, 3), (3, 4), (3, 5), (4, 6), (4, 7), (4, 8)]
    print(solve(8, edges))
