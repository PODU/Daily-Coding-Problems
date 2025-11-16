# Day 606: Count all open knight's tours: DFS backtracking with a bitmask visited
# set and precomputed move lists; start squares are grouped by the board's 8
# dihedral symmetries so each orbit is searched once. N=5 -> 1728.
# Time O(8^(N*N)) worst, Space O(N*N).

MOVES = [(1, 2), (1, -2), (-1, 2), (-1, -2),
         (2, 1), (2, -1), (-2, 1), (-2, -1)]


def knight_tours(n):
    if n == 0:
        return 0
    total_cells = n * n
    full = (1 << total_cells) - 1
    adj = []
    for x in range(n):
        for y in range(n):
            moves = []
            for dx, dy in MOVES:
                nx, ny = x + dx, y + dy
                if 0 <= nx < n and 0 <= ny < n:
                    idx = nx * n + ny
                    moves.append((idx, 1 << idx))
            adj.append(tuple(moves))

    def dfs(cell, visited):
        if visited == full:
            return 1
        count = 0
        for idx, bit in adj[cell]:
            if not visited & bit:
                count += dfs(idx, visited | bit)
        return count

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


if __name__ == "__main__":
    print(knight_tours(5))
