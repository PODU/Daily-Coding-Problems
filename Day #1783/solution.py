# Day 1783: Count regions split by '/','\\',' ' via Union-Find: each cell = 4 triangles (T,R,B,L).
# Union within cell per char and across neighbors. Time O(R*C*alpha), Space O(R*C).

class DSU:
    def __init__(self, n):
        self.p = list(range(n))

    def find(self, x):
        while self.p[x] != x:
            self.p[x] = self.p[self.p[x]]
            x = self.p[x]
        return x

    def union(self, a, b):
        self.p[self.find(a)] = self.find(b)


def regions(grid):
    R, C = len(grid), len(grid[0])
    dsu = DSU(4 * R * C)

    def idx(r, c, t):
        return 4 * (r * C + c) + t

    for r in range(R):
        for c in range(C):
            ch = grid[r][c]
            if ch == ' ':
                dsu.union(idx(r, c, 0), idx(r, c, 1))
                dsu.union(idx(r, c, 1), idx(r, c, 2))
                dsu.union(idx(r, c, 2), idx(r, c, 3))
            elif ch == '/':
                dsu.union(idx(r, c, 0), idx(r, c, 3))
                dsu.union(idx(r, c, 1), idx(r, c, 2))
            else:  # '\\'
                dsu.union(idx(r, c, 0), idx(r, c, 1))
                dsu.union(idx(r, c, 2), idx(r, c, 3))
            if c + 1 < C:
                dsu.union(idx(r, c, 1), idx(r, c + 1, 3))
            if r + 1 < R:
                dsu.union(idx(r, c, 2), idx(r + 1, c, 0))

    return len({dsu.find(i) for i in range(4 * R * C)})


def main():
    grid = [
        "\\    /",
        " \\  / ",
        "  \\/  ",
    ]
    print(regions(grid))  # 3


if __name__ == "__main__":
    main()
