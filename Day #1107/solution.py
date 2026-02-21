# Day 1107: Max edges removable so every component has an even node count.
# DFS subtree sizes; every non-root node with an even-sized subtree => one cuttable edge.
# Time: O(V+E). Space: O(V).
import sys


def max_edges_removed(n, edges, root=1):
    adj = [[] for _ in range(n + 1)]
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)
    answer = [0]

    def dfs(u, parent):
        size = 1
        for v in adj[u]:
            if v != parent:
                size += dfs(v, u)
        if parent != -1 and size % 2 == 0:
            answer[0] += 1
        return size

    dfs(root, -1)
    return answer[0]


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    edges = [(1, 2), (1, 3), (3, 4), (3, 5), (4, 6), (4, 7), (4, 8)]
    print(max_edges_removed(8, edges))  # 2
