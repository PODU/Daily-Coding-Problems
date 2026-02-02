# Day 1003: Transitive closure of a graph (adjacency list -> reachability matrix).
# Run a DFS from each vertex marking everything reachable. O(V * (V + E)) time,
# O(V^2) space for the result matrix.

def transitive_closure(graph):
    n = len(graph)
    M = [[0] * n for _ in range(n)]

    def dfs(start, u):
        for v in graph[u]:
            if M[start][v] == 0:
                M[start][v] = 1
                dfs(start, v)

    for s in range(n):
        M[s][s] = 1
        dfs(s, s)
    return M


if __name__ == "__main__":
    graph = [[0, 1, 3], [1, 2], [2], [3]]
    for row in transitive_closure(graph):
        print(row)
    # [1, 1, 1, 1]
    # [0, 1, 1, 0]
    # [0, 0, 1, 0]
    # [0, 0, 0, 1]
