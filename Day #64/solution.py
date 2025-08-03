# Day 64: Count open knight's tours on N x N board via backtracking DFS.
# Bitmask visited set, precomputed neighbor lists, and board symmetry
# (each start square counted once per orbit under the 8 dihedral symmetries).
# Time exponential, Space O(N^2).

MOVES = [(-2,-1),(-2,1),(-1,-2),(-1,2),(1,-2),(1,2),(2,-1),(2,1)]


def count_tours(n):
    full = (1 << (n * n)) - 1
    neighbors = []
    for r in range(n):
        for c in range(n):
            cell = []
            for dr, dc in MOVES:
                nr, nc = r + dr, c + dc
                if 0 <= nr < n and 0 <= nc < n:
                    idx = nr * n + nc
                    cell.append((idx, 1 << idx))
            neighbors.append(tuple(cell))

    def dfs(cell, vis):
        if vis == full:
            return 1
        res = 0
        for idx, bit in neighbors[cell]:
            if not vis & bit:
                res += dfs(idx, vis | bit)
        return res

    m = n - 1
    total = 0
    seen = set()
    for r in range(n):
        for c in range(n):
            if (r, c) in seen:
                continue
            orbit = {(r, c), (c, r), (r, m - c), (m - c, r),
                     (m - r, c), (c, m - r), (m - r, m - c), (m - c, m - r)}
            seen |= orbit
            start = r * n + c
            total += len(orbit) * dfs(start, 1 << start)
    return total


if __name__ == "__main__":
    print(count_tours(5))
