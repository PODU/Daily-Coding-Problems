# Day 698: Count regions a grid of '/'/'\\'/' ' is divided into.
# Approach: split each cell into 4 triangles (top,right,bottom,left) and union by
# the slash type plus across neighbors (Union-Find). Time O(R*C*a), Space O(R*C).


def regions(grid):
    R = len(grid)
    C = max((len(s) for s in grid), default=0)
    grid = [s.ljust(C) for s in grid]
    parent = list(range(R * C * 4))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def uni(a, b):
        parent[find(a)] = find(b)

    for r in range(R):
        for c in range(C):
            base = (r * C + c) * 4  # 0=T,1=R,2=B,3=L
            ch = grid[r][c]
            if ch == '/':
                uni(base, base + 3); uni(base + 1, base + 2)
            elif ch == '\\':
                uni(base, base + 1); uni(base + 2, base + 3)
            else:
                uni(base, base + 1); uni(base + 1, base + 2); uni(base + 2, base + 3)
            if c + 1 < C:
                uni(base + 1, (r * C + c + 1) * 4 + 3)
            if r + 1 < R:
                uni(base + 2, ((r + 1) * C + c) * 4)

    return sum(1 for i in range(R * C * 4) if find(i) == i)


if __name__ == "__main__":
    grid = ["\\    /", " \\  /", "  \\/"]
    print(regions(grid))  # 3
