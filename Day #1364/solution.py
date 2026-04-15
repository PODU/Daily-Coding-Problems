# Day 1364: Topological DP over DAG: dp[node][c] = max count of letter c on a path ending at node.
# Kahn's algorithm detects cycles (return None). Time O((V+E)*26), Space O(V*26).
from collections import deque


def largest_path_value(colors, edges):
    """Return largest path value, or None if a cycle exists (null case)."""
    n = len(colors)
    adj = [[] for _ in range(n)]
    indeg = [0] * n
    for a, b in edges:
        adj[a].append(b)
        indeg[b] += 1

    dp = [[0] * 26 for _ in range(n)]
    q = deque(i for i in range(n) if indeg[i] == 0)

    processed = 0
    ans = 0
    while q:
        u = q.popleft()
        processed += 1
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

    if processed < n:
        return None  # cycle -> null
    return ans


def main():
    colors = "ABACA"
    edges = [(0, 1), (0, 2), (2, 3), (3, 4)]
    res = largest_path_value(colors, edges)
    print("null" if res is None else res)


if __name__ == "__main__":
    main()
