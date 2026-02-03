# Day 1015: Largest path value in a directed graph: DFS topological memo + color cycle detection.
# dp[u][c] = max count of letter c on a path ending at u. Cycle -> None (null). O((n+m)*26) time, O(n*26) space.
import sys

def largest_path_value(colors, edges):
    n = len(colors)
    adj = [[] for _ in range(n)]
    for u, v in edges:
        adj[u].append(v)
    dp = [[0] * 26 for _ in range(n)]
    state = [0] * n  # 0 unvisited, 1 in-stack, 2 done
    sys.setrecursionlimit(10 ** 6)

    def dfs(u):
        state[u] = 1
        for v in adj[u]:
            if state[v] == 1:
                return False        # back edge -> cycle
            if state[v] == 0 and not dfs(v):
                return False
        for v in adj[u]:
            for c in range(26):
                if dp[v][c] > dp[u][c]:
                    dp[u][c] = dp[v][c]
        dp[u][ord(colors[u]) - 65] += 1
        state[u] = 2
        return True

    for i in range(n):
        if state[i] == 0 and not dfs(i):
            return None
    return max(max(row) for row in dp)


def show(x):
    return "null" if x is None else x


if __name__ == "__main__":
    print(show(largest_path_value("ABACA", [(0, 1), (0, 2), (2, 3), (3, 4)])))
    print(show(largest_path_value("A", [(0, 0)])))
