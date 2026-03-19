# Day 1232: Count regions carved by '/','\\' in a grid. Split each cell into 4 triangles
# (top,right,bottom,left) and union by Union-Find. Time O(n*m a(n)), Space O(n*m).


class DSU:
    def __init__(self, n):
        self.p = list(range(n))
        self.count = n

    def find(self, x):
        while self.p[x] != x:
            self.p[x] = self.p[self.p[x]]
            x = self.p[x]
        return x

    def union(self, a, b):
        ra, rb = self.find(a), self.find(b)
        if ra != rb:
            self.p[ra] = rb
            self.count -= 1


def regions(grid):
    rows = len(grid)
    cols = len(grid[0]) if rows else 0
    dsu = DSU(4 * rows * cols)

    def idx(r, c, k):
        return 4 * (r * cols + c) + k  # k: 0 top,1 right,2 bottom,3 left

    for r in range(rows):
        for c in range(cols):
            ch = grid[r][c]
            t, ri, b, le = (idx(r, c, k) for k in range(4))
            if ch == '/':
                dsu.union(t, le)
                dsu.union(ri, b)
            elif ch == '\\':
                dsu.union(t, ri)
                dsu.union(le, b)
            else:
                dsu.union(t, ri)
                dsu.union(ri, b)
                dsu.union(b, le)
            if c + 1 < cols:
                dsu.union(ri, idx(r, c + 1, 3))
            if r + 1 < rows:
                dsu.union(b, idx(r + 1, c, 0))
    return dsu.count


if __name__ == "__main__":
    grid = [
        "\\    /",
        " \\  / ",
        "  \\/  ",
    ]
    print(regions(grid))
