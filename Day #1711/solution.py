# Day 1711: Transitive closure via DFS from each vertex. Time O(V*(V+E)), Space O(V^2).
def transitive_closure(graph):
    n = len(graph)
    M = [[0] * n for _ in range(n)]
    for s in range(n):
        vis = [False] * n
        stack = [s]
        while stack:
            u = stack.pop()
            if vis[u]:
                continue
            vis[u] = True
            M[s][u] = 1
            for v in graph[u]:
                if not vis[v]:
                    stack.append(v)
    return M


if __name__ == "__main__":
    graph = [[0, 1, 3], [1, 2], [2], [3]]
    for row in transitive_closure(graph):
        print("[" + ", ".join(str(x) for x in row) + "]")
