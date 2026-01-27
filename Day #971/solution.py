# Day 971: Rotate N x N matrix 90 degrees clockwise in place.
# Approach: transpose then reverse each row. Time O(N^2), Space O(1).


def rotate(m):
    n = len(m)
    for i in range(n):
        for j in range(i + 1, n):
            m[i][j], m[j][i] = m[j][i], m[i][j]
    for row in m:
        row.reverse()


if __name__ == '__main__':
    m = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    rotate(m)
    for row in m:
        print(' '.join(map(str, row)))
    # 7 4 1 / 8 5 2 / 9 6 3
