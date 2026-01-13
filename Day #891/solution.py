# Day 891: Region cutting by slashes: split each cell into 4 triangles, union-find.
# Time: O(N*M * alpha), Space: O(N*M).
def regions(g):
    n, m = len(g), len(g[0])
    par = list(range(n * m * 4))

    def find(x):
        while par[x] != x:
            par[x] = par[par[x]]
            x = par[x]
        return x

    def uni(a, b):
        par[find(a)] = find(b)

    for r in range(n):
        for c in range(m):
            base = (r * m + c) * 4
            ch = g[r][c]
            if ch == '/':
                uni(base + 0, base + 3); uni(base + 1, base + 2)
            elif ch == '\\':
                uni(base + 0, base + 1); uni(base + 2, base + 3)
            else:
                uni(base + 0, base + 1); uni(base + 1, base + 2); uni(base + 2, base + 3)
            if c + 1 < m:
                uni(base + 1, ((r * m + c + 1) * 4) + 3)
            if r + 1 < n:
                uni(base + 2, (((r + 1) * m + c) * 4) + 0)

    return sum(1 for i in range(n * m * 4) if find(i) == i)


if __name__ == "__main__":
    g = [
        "\\    /",
        " \\  / ",
        "  \\/  ",
    ]
    print(regions(g))
