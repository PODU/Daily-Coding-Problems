# Day 392: Island perimeter: +4 per land cell, -2 per adjacent right/down land pair. O(R*C) time, O(1) space.

def island_perimeter(g):
    R = len(g)
    C = len(g[0]) if R else 0
    per = 0
    for r in range(R):
        for c in range(C):
            if g[r][c] == 1:
                per += 4
                if c + 1 < C and g[r][c + 1] == 1:
                    per -= 2
                if r + 1 < R and g[r + 1][c] == 1:
                    per -= 2
    return per


if __name__ == "__main__":
    grid = [[0, 1, 1, 0], [1, 1, 1, 0], [0, 1, 1, 0], [0, 0, 1, 0]]
    print(island_perimeter(grid))
