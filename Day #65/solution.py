# Day 65: Spiral order print via boundary shrinking (top/bottom/left/right). Time O(N*M), Space O(1) extra.

def spiral(m):
    top, bottom = 0, len(m) - 1
    left, right = 0, len(m[0]) - 1
    out = []
    while top <= bottom and left <= right:
        for j in range(left, right + 1):
            out.append(m[top][j])
        top += 1
        for i in range(top, bottom + 1):
            out.append(m[i][right])
        right -= 1
        if top <= bottom:
            for j in range(right, left - 1, -1):
                out.append(m[bottom][j])
            bottom -= 1
        if left <= right:
            for i in range(bottom, top - 1, -1):
                out.append(m[i][left])
            left += 1
    return out


if __name__ == "__main__":
    matrix = [
        [1, 2, 3, 4, 5],
        [6, 7, 8, 9, 10],
        [11, 12, 13, 14, 15],
        [16, 17, 18, 19, 20],
    ]
    for v in spiral(matrix):
        print(v)
