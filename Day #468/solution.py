# Day 468: Rotate NxN matrix 90 deg clockwise in place: transpose then reverse each row.
# Time: O(n^2), Space: O(1).
def rotate(m):
    n = len(m)
    for i in range(n):
        for j in range(i + 1, n):
            m[i][j], m[j][i] = m[j][i], m[i][j]
    for row in m:
        row.reverse()


if __name__ == "__main__":
    m = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    rotate(m)
    n = len(m)
    lines = []
    for i in range(n):
        prefix = "[[" if i == 0 else " ["
        body = ", ".join(str(x) for x in m[i])
        suffix = "]" if i < n - 1 else "]]"
        lines.append(prefix + body + suffix)
    print(",\n".join(lines))
