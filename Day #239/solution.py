# Day 239: Android unlock patterns of length N: DFS over the 1..9 keypad, tracking visited keys and a
# "skip" matrix (key jumped over must already be visited). Symmetry reduces work to 3 starts.
# Time: O(9!) worst case, Space: O(9).

skip = [[0] * 10 for _ in range(10)]
skip[1][3] = skip[3][1] = 2
skip[1][7] = skip[7][1] = 4
skip[3][9] = skip[9][3] = 6
skip[7][9] = skip[9][7] = 8
skip[1][9] = skip[9][1] = skip[3][7] = skip[7][3] = 5
skip[2][8] = skip[8][2] = 5
skip[4][6] = skip[6][4] = 5


def dfs(cur, remaining, visited):
    if remaining == 0:
        return 1
    visited[cur] = True
    count = 0
    for nxt in range(1, 10):
        mid = skip[cur][nxt]
        if not visited[nxt] and (mid == 0 or visited[mid]):
            count += dfs(nxt, remaining - 1, visited)
    visited[cur] = False
    return count


def patterns(n):
    v = [False] * 10
    return dfs(1, n - 1, v) * 4 + dfs(2, n - 1, v) * 4 + dfs(5, n - 1, v)


if __name__ == "__main__":
    for n in range(1, 10):
        print(f"N={n}: {patterns(n)}")
    # e.g. N=1 -> 9, N=4 -> 1624 (standard canonical counts)
