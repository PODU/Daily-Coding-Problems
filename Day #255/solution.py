# Day 255: Transitive closure: DFS from each vertex marking reachable nodes (incl self).
# Time O(V*(V+E)), Space O(V^2) for the matrix.

def dfs(u, g, row):
    row[u] = 1
    for v in g[u]:
        if not row[v]:
            dfs(v, g, row)


if __name__ == "__main__":
    graph = [[0, 1, 3], [1, 2], [2], [3]]
    n = len(graph)
    M = [[0] * n for _ in range(n)]
    for i in range(n):
        dfs(i, graph, M[i])
    for row in M:
        print("[" + ", ".join(map(str, row)) + "]")
