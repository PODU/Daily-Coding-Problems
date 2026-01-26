# Day 962: Count knight's tours on an N x N board (visit every square once).
# Approach: DFS backtracking with a bitmask visited set and precomputed move
# lists; symmetric start squares (8 dihedral board symmetries) are searched
# once per orbit. Time O(8^(N^2)) worst, Space O(N^2).

MOVES = [(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)]


def count_tours(n: int) -> int:
    full = (1 << (n * n)) - 1
    adj = []
    for x in range(n):
        for y in range(n):
            cell = []
            for ddx, ddy in MOVES:
                nx, ny = x + ddx, y + ddy
                if 0 <= nx < n and 0 <= ny < n:
                    idx = nx * n + ny
                    cell.append((idx, 1 << idx))
            adj.append(tuple(cell))

    def dfs(cell, vis):
        if vis == full:
            return 1
        t = 0
        for idx, bit in adj[cell]:
            if not vis & bit:
                t += dfs(idx, vis | bit)
        return t

    m = n - 1
    total = 0
    seen = set()
    for i in range(n):
        for j in range(n):
            if (i, j) in seen:
                continue
            orbit = {(i, j), (j, i), (i, m - j), (m - j, i),
                     (m - i, j), (j, m - i), (m - i, m - j), (m - j, m - i)}
            seen |= orbit
            start = i * n + j
            total += len(orbit) * dfs(start, 1 << start)
    return total


if __name__ == '__main__':
    print(count_tours(5))  # 1728
