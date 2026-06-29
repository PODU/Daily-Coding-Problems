# Day 1738: Topological-order DP over DAG: dp[node][c]=max count of letter c on path from node.
# Kahn's algorithm; cycle => None. Time O((n+m)*26), Space O(n*26).
from collections import deque


def largest_path_value(s, edges):
    n = len(s)
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
        cu = ord(s[u]) - ord('A')
        dp[u][cu] += 1
        ans = max(ans, dp[u][cu])
        for v in adj[u]:
            for c in range(26):
                if dp[u][c] > dp[v][c]:
                    dp[v][c] = dp[u][c]
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return None if seen < n else ans


def fmt(v):
    return "null" if v is None else str(v)


if __name__ == "__main__":
    print(fmt(largest_path_value("ABACA", [(0, 1), (0, 2), (2, 3), (3, 4)])))
    print(fmt(largest_path_value("A", [(0, 0)])))
