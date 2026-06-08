# Day 1628: Rotate N x N matrix 90 degrees clockwise in place.
# Transpose then reverse each row. Time O(N^2), Space O(1).
def rotate(m):
    n = len(m)
    for i in range(n):
        for j in range(i + 1, n):
            m[i][j], m[j][i] = m[j][i], m[i][j]
    for row in m:
        row.reverse()
    return m


if __name__ == "__main__":
    mat = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    rotate(mat)
    for row in mat:
        print(" ".join(map(str, row)))
