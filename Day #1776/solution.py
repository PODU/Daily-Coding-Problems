# Day 1776: Count valid Android unlock patterns of length N.
# DFS with symmetry + jump-over rules. Time O(N!) bounded by 9!, Space O(9).

skip = {}
for a, b, m in [(1, 3, 2), (1, 7, 4), (3, 9, 6), (7, 9, 8),
                (1, 9, 5), (3, 7, 5), (2, 8, 5), (4, 6, 5)]:
    skip[(a, b)] = m
    skip[(b, a)] = m

visited = [False] * 10


def dfs(cur, remaining):
    if remaining == 0:
        return 1
    visited[cur] = True
    cnt = 0
    for nxt in range(1, 10):
        if visited[nxt]:
            continue
        mid = skip.get((cur, nxt), 0)
        if mid == 0 or visited[mid]:
            cnt += dfs(nxt, remaining - 1)
    visited[cur] = False
    return cnt


def count_patterns(n):
    return 4 * dfs(1, n - 1) + 4 * dfs(2, n - 1) + dfs(5, n - 1)


if __name__ == "__main__":
    for n in range(1, 10):
        print(f"N={n}: {count_patterns(n)}")
