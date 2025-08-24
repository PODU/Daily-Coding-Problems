# Day 168: Rotate NxN 90 clockwise in place: transpose then reverse each row. O(n^2) time, O(1) extra space.

def rotate(m):
    n = len(m)
    for i in range(n):
        for j in range(i + 1, n):
            m[i][j], m[j][i] = m[j][i], m[i][j]
    for row in m:
        row.reverse()


def main():
    m = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    rotate(m)
    for row in m:
        print("[" + ", ".join(str(x) for x in row) + "]")


if __name__ == "__main__":
    main()
