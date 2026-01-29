# Day 985: Count valid Android unlock patterns of length N via backtracking with a
# skip[a][b] over-jump table; use 8-fold symmetry of corners/edges/center.
# Time: O(N * 9!) worst-case bounded by symmetry; Space: O(9).

skip = [[0] * 10 for _ in range(10)]
used = [False] * 10


def dfs(cur, remaining):
    if remaining == 0:
        return 1
    used[cur] = True
    count = 0
    for nxt in range(1, 10):
        if used[nxt]:
            continue
        mid = skip[cur][nxt]
        if mid == 0 or used[mid]:
            count += dfs(nxt, remaining - 1)
    used[cur] = False
    return count


def count_patterns(n):
    for i in range(10):
        used[i] = False
    return 4 * dfs(1, n - 1) + 4 * dfs(2, n - 1) + dfs(5, n - 1)


if __name__ == "__main__":
    skip[1][3] = skip[3][1] = 2
    skip[1][7] = skip[7][1] = 4
    skip[3][9] = skip[9][3] = 6
    skip[7][9] = skip[9][7] = 8
    skip[1][9] = skip[9][1] = skip[3][7] = skip[7][3] = 5
    skip[2][8] = skip[8][2] = 5
    skip[4][6] = skip[6][4] = 5

    for n in range(1, 10):
        print("N=%d: %d" % (n, count_patterns(n)))
