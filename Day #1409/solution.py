# Day 1409: Transitive closure: each input row is [node, neighbors...]. DFS from every node
# marks all reachable vertices (incl. itself). Time O(V*(V+E)), Space O(V^2).

def transitive_closure(graph):
    n = len(graph)
    adj = [[] for _ in range(n)]
    for r in graph:
        node = r[0]
        adj[node].extend(r[1:])
    M = [[0] * n for _ in range(n)]

    def dfs(u, row):
        row[u] = 1
        for v in adj[u]:
            if not row[v]:
                dfs(v, row)

    for i in range(n):
        dfs(i, M[i])
    return M


if __name__ == "__main__":
    graph = [[0, 1, 3], [1, 2], [2], [3]]
    for row in transitive_closure(graph):
        print(row)
