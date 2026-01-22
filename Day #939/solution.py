# Day 939: Print an N x M matrix in clockwise spiral order.
# Shrink four boundaries layer by layer. Time O(N*M), Space O(1) extra.


def spiral(m):
    if not m:
        return
    top, bottom = 0, len(m) - 1
    left, right = 0, len(m[0]) - 1
    while top <= bottom and left <= right:
        for c in range(left, right + 1):
            print(m[top][c])
        top += 1
        for r in range(top, bottom + 1):
            print(m[r][right])
        right -= 1
        if top <= bottom:
            for c in range(right, left - 1, -1):
                print(m[bottom][c])
            bottom -= 1
        if left <= right:
            for r in range(bottom, top - 1, -1):
                print(m[r][left])
            left += 1


if __name__ == "__main__":
    matrix = [
        [1, 2, 3, 4, 5],
        [6, 7, 8, 9, 10],
        [11, 12, 13, 14, 15],
        [16, 17, 18, 19, 20],
    ]
    spiral(matrix)  # 1 2 3 4 5 10 15 20 19 18 17 16 11 6 7 8 9 14 13 12
