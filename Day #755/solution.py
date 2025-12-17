# Day 755: Largest value path in a directed graph.
# Topological DP: dp[u][c] = max count of letter c on a path starting at u.
# Cycle -> value is infinite -> None. Time: O((n+m)*26), Space: O(n*26).
from collections import deque


def largest_path_value(letters, edges):
    n = len(letters)
    adj = [[] for _ in range(n)]
    indeg = [0] * n
    for i, j in edges:
        adj[i].append(j)
        indeg[j] += 1

    q = deque(i for i in range(n) if indeg[i] == 0)
    topo = []
    while q:
        u = q.popleft()
        topo.append(u)
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    if len(topo) < n:
        return None  # cycle -> infinite

    dp = [[0] * 26 for _ in range(n)]
    for i in range(n):
        dp[i][ord(letters[i]) - 65] = 1

    best = 0
    for u in reversed(topo):
        uc = ord(letters[u]) - 65
        for v in adj[u]:
            for c in range(26):
                add = dp[v][c] + (1 if c == uc else 0)
                if add > dp[u][c]:
                    dp[u][c] = add
        best = max(best, max(dp[u]))
    return best


if __name__ == "__main__":
    letters = "ABACA"
    edges = [(0, 1), (0, 2), (2, 3), (3, 4)]
    res = largest_path_value(letters, edges)
    print("null" if res is None else res)  # 3
