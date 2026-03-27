# Day 1272: Determine whether a matrix is Toeplitz (each TL->BR diagonal is constant).
# Compare every cell with its upper-left neighbor. O(N*M) time, O(1) space.
from typing import List


def is_toeplitz(m: List[List[int]]) -> bool:
    return all(
        m[i][j] == m[i - 1][j - 1]
        for i in range(1, len(m))
        for j in range(1, len(m[i]))
    )


if __name__ == "__main__":
    matrix = [[1, 2, 3, 4, 8], [5, 1, 2, 3, 4], [4, 5, 1, 2, 3], [7, 4, 5, 1, 2]]
    print(str(is_toeplitz(matrix)).lower())
