# Day 1269: Rotate N x N matrix 90 degrees clockwise, in place.
# Transpose then reverse each row. O(n^2) time, O(1) extra space.
from typing import List


def rotate(m: List[List[int]]) -> None:
    n = len(m)
    for i in range(n):
        for j in range(i + 1, n):
            m[i][j], m[j][i] = m[j][i], m[i][j]
    for row in m:
        row.reverse()


if __name__ == "__main__":
    matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    rotate(matrix)
    print(matrix)
