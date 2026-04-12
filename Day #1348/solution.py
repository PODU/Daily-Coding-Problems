# Day 1348: Weighted tree diameter: DFS, at each node track two largest downward child path sums;
# diameter = max over nodes of (sum of two largest). Time O(V+E), Space O(V+E).
import sys


def tree_diameter(n, edges):
    adj = [[] for _ in range(n)]
    for a, b, w in edges:
        adj[a].append((b, w))
        adj[b].append((a, w))
    best = [0]

    def dfs(u, parent):
        max1 = max2 = 0  # two largest downward path sums
        for v, w in adj[u]:
            if v == parent:
                continue
            down = dfs(v, u) + w
            if down > max1:
                max1, max2 = down, max1
            elif down > max2:
                max2 = down
        best[0] = max(best[0], max1 + max2)
        return max1

    dfs(0, -1)
    return best[0]


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    # a..h -> 0..7
    edges = [
        (0, 1, 3),  # a-b
        (0, 2, 5),  # a-c
        (0, 3, 8),  # a-d
        (3, 4, 2),  # d-e
        (3, 5, 4),  # d-f
        (4, 6, 1),  # e-g
        (4, 7, 1),  # e-h
    ]
    print(tree_diameter(8, edges))
