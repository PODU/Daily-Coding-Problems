# Day 1468: Knight's tour counting via DFS backtracking with a bitmask visited
# set and precomputed neighbor lists; start cells are deduplicated by the 8
# dihedral board symmetries. Time: exponential (bounded by tour search);
# Space: O(N^2) neighbor lists + recursion.

DX = (1, 1, -1, -1, 2, 2, -2, -2)
DY = (2, -2, 2, -2, 1, -1, 1, -1)


def count_tours(n):
    full = (1 << (n * n)) - 1
    neighbors = []
    for x in range(n):
        for y in range(n):
            cell = []
            for d in range(8):
                nx, ny = x + DX[d], y + DY[d]
                if 0 <= nx < n and 0 <= ny < n:
                    idx = nx * n + ny
                    cell.append((idx, 1 << idx))
            neighbors.append(tuple(cell))

    def dfs(cell, visited):
        if visited == full:
            return 1
        total = 0
        for idx, bit in neighbors[cell]:
            if not visited & bit:
                total += dfs(idx, visited | bit)
        return total

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
    print(count_tours(1))
    print(count_tours(5))
