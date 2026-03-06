# Day 1162: Root tree, DFS subtree sizes; count non-root nodes with even subtree size (cuttable parent edges). O(n) time, O(n) space.
import sys
sys.setrecursionlimit(10000)

def solve(n, edges):
    adj = [[] for _ in range(n + 1)]
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)
    answer = 0

    def dfs(u, parent):
        nonlocal answer
        size = 1
        for v in adj[u]:
            if v != parent:
                size += dfs(v, u)
        if parent != -1 and size % 2 == 0:
            answer += 1
        return size

    dfs(1, -1)
    return answer

if __name__ == "__main__":
    edges = [(1, 2), (1, 3), (3, 4), (3, 5), (4, 6), (4, 7), (4, 8)]
    print(solve(8, edges))
