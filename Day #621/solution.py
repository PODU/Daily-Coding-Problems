# Day 621: Tree diameter with edge weights: DFS returning max downward path; global best
# = sum of two largest child paths. Time O(N), Space O(N).
import sys


def longest_path(n, edges):
    adj = [[] for _ in range(n)]
    for u, v, w in edges:
        adj[u].append((v, w))
        adj[v].append((u, w))
    best = 0

    def dfs(u, parent):
        nonlocal best
        max1 = max2 = 0
        for v, w in adj[u]:
            if v == parent:
                continue
            path = dfs(v, u) + w
            if path > max1:
                max1, max2 = path, max1
            elif path > max2:
                max2 = path
        best = max(best, max1 + max2)
        return max1

    dfs(0, -1)
    return best


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    # a0 b1 c2 d3 e4 f5 g6 h7
    edges = [(0, 1, 3), (0, 2, 5), (0, 3, 8),
             (3, 4, 2), (3, 5, 4), (4, 6, 1), (4, 7, 1)]
    print(longest_path(8, edges))
