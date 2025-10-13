# Day 423: Transitive closure via DFS from each vertex. O(V*(V+E)) time, O(V^2) space.
# M[i][j] = 1 iff j is reachable from i (each vertex reaches itself).
def transitive_closure(g):
    n = len(g)
    M = [[0] * n for _ in range(n)]

    def dfs(src, u):
        M[src][u] = 1
        for v in g[u]:
            if not M[src][v]:
                dfs(src, v)

    for i in range(n):
        dfs(i, i)
    return M


if __name__ == "__main__":
    g = [[0, 1, 3], [1, 2], [2], [3]]
    for row in transitive_closure(g):
        print("[" + ", ".join(map(str, row)) + "]")
