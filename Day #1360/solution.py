# Day 1360: Count Android unlock patterns of length N via DFS backtracking with a skip
# (midpoint) table; symmetry over corners/edges. Time O(N!) worst, Space O(N).
import sys

skip = [[0] * 10 for _ in range(10)]
skip[1][3] = skip[3][1] = 2
skip[1][7] = skip[7][1] = 4
skip[3][9] = skip[9][3] = 6
skip[7][9] = skip[9][7] = 8
skip[1][9] = skip[9][1] = skip[2][8] = skip[8][2] = 5
skip[3][7] = skip[7][3] = skip[4][6] = skip[6][4] = 5


def dfs(cur, remaining, visited):
    if remaining == 0:
        return 1
    visited[cur] = True
    count = 0
    for nxt in range(1, 10):
        if not visited[nxt] and (skip[cur][nxt] == 0 or visited[skip[cur][nxt]]):
            count += dfs(nxt, remaining - 1, visited)
    visited[cur] = False
    return count


def count_patterns(n):
    visited = [False] * 10
    corner = dfs(1, n - 1, visited)
    edge = dfs(2, n - 1, visited)
    center = dfs(5, n - 1, visited)
    return corner * 4 + edge * 4 + center


if __name__ == "__main__":
    print(count_patterns(4))
