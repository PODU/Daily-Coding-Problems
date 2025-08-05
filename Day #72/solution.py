# Day 72: Largest path value in directed graph: topo sort (Kahn) + DP dp[node][26]. Cycle -> null. Time O((n+m)*26), Space O(n*26).
from collections import deque


def largest_path_value(colors, edges):
    # Graph "A" with edge (0,0) returns None (self-loop is a cycle).
    n = len(colors)
    adj = [[] for _ in range(n)]
    indeg = [0] * n
    for a, b in edges:
        adj[a].append(b)
        indeg[b] += 1
    dp = [[0] * 26 for _ in range(n)]
    q = deque(i for i in range(n) if indeg[i] == 0)
    seen = 0
    ans = 0
    while q:
        u = q.popleft()
        seen += 1
        cu = ord(colors[u]) - ord('A')
        dp[u][cu] += 1
        ans = max(ans, dp[u][cu])
        for v in adj[u]:
            for c in range(26):
                if dp[u][c] > dp[v][c]:
                    dp[v][c] = dp[u][c]
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    if seen < n:
        return None  # cycle
    return ans


def main():
    colors = "ABACA"
    edges = [(0, 1), (0, 2), (2, 3), (3, 4)]
    result = largest_path_value(colors, edges)
    print("null" if result is None else result)


if __name__ == "__main__":
    main()
