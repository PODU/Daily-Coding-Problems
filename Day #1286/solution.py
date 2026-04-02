# Day 1286: Find all bridges in an undirected graph (Tarjan's low-link DFS).
# Time O(V + E), Space O(V + E). Iterative DFS to avoid recursion limits.
def find_bridges(n, edges):
    adj = [[] for _ in range(n)]
    for u, v in edges:
        adj[u].append(v)
        adj[v].append(u)
    disc = [0] * n
    low = [0] * n
    timer = [0]
    bridges = []

    def dfs(start):
        # stack of (node, parent, iterator index)
        stack = [(start, -1, 0)]
        timer[0] += 1
        disc[start] = low[start] = timer[0]
        skipped = [False] * n
        while stack:
            u, parent, i = stack[-1]
            if i < len(adj[u]):
                stack[-1] = (u, parent, i + 1)
                v = adj[u][i]
                if v == parent and not skipped[u]:
                    skipped[u] = True
                    continue
                if disc[v] == 0:
                    timer[0] += 1
                    disc[v] = low[v] = timer[0]
                    stack.append((v, u, 0))
                else:
                    low[u] = min(low[u], disc[v])
            else:
                stack.pop()
                if stack:
                    p = stack[-1][0]
                    low[p] = min(low[p], low[u])
                    if low[u] > disc[p]:
                        bridges.append((min(p, u), max(p, u)))

    for s in range(n):
        if disc[s] == 0:
            dfs(s)
    return sorted(bridges)


if __name__ == "__main__":
    edges = [(0, 1), (1, 2), (2, 0), (1, 3), (3, 4)]
    for u, v in find_bridges(5, edges):
        print(f"{u} - {v}")
