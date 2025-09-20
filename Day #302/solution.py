# Day 302: Count regions divided by slashes. Split each cell into 4 triangles,
# union-find adjacent triangles. Time O(N*M*alpha), space O(N*M).
def count_regions(grid):
    n = len(grid)
    M = max(len(r) for r in grid)
    g = [r.ljust(M) for r in grid]
    parent = list(range(n * M * 4))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def union(a, b):
        parent[find(a)] = find(b)

    def idx(i, j, t):  # 0=top,1=right,2=bottom,3=left
        return (i * M + j) * 4 + t

    for i in range(n):
        for j in range(M):
            c = g[i][j]
            if c == '/':
                union(idx(i, j, 0), idx(i, j, 3)); union(idx(i, j, 1), idx(i, j, 2))
            elif c == '\\':
                union(idx(i, j, 0), idx(i, j, 1)); union(idx(i, j, 2), idx(i, j, 3))
            else:
                union(idx(i, j, 0), idx(i, j, 1))
                union(idx(i, j, 1), idx(i, j, 2))
                union(idx(i, j, 2), idx(i, j, 3))
            if j + 1 < M:
                union(idx(i, j, 1), idx(i, j + 1, 3))
            if i + 1 < n:
                union(idx(i, j, 2), idx(i + 1, j, 0))

    return sum(1 for x in range(n * M * 4) if find(x) == x)


if __name__ == "__main__":
    grid = ["\\    /", " \\  /", "  \\/"]
    print(count_regions(grid))  # 3
