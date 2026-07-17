# Day 1824: Count knight's tours on NxN via DFS backtracking with a bitmask
# visited set and precomputed move lists; symmetric start cells (8 dihedral
# board symmetries) are searched once per orbit. Worst-case exponential;
# fine for small N (N=5 -> 1728).

DR = (-2, -2, -1, -1, 1, 1, 2, 2)
DC = (-1, 1, -2, 2, -2, 2, -1, 1)


def count_tours(n):
    full = (1 << (n * n)) - 1
    adj = []
    for r in range(n):
        for c in range(n):
            cell = []
            for k in range(8):
                nr, nc = r + DR[k], c + DC[k]
                if 0 <= nr < n and 0 <= nc < n:
                    idx = nr * n + nc
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
    print(count_tours(5))  # 1728
