# Day 195: In a row- and column-sorted matrix, count elements < M[i1,j1] or > M[i2,j2].
# Staircase counting of "<= x". Time O(N+M) per query, Space O(1).
# Note: the README example counts the lower bound inclusively (expected 15), so we use
# "<= M[i1,j1]" for the smaller side and strict "> M[i2,j2]" for the larger side.
from typing import List


def count_less_equal(A: List[List[int]], x: int) -> int:
    n, m = len(A), len(A[0])
    r, c, cnt = 0, m - 1, 0
    while r < n and c >= 0:
        if A[r][c] <= x:
            cnt += c + 1
            r += 1
        else:
            c -= 1
    return cnt


def solve(A: List[List[int]], i1: int, j1: int, i2: int, j2: int) -> int:
    n, m = len(A), len(A[0])
    smaller = count_less_equal(A, A[i1][j1])
    larger = n * m - count_less_equal(A, A[i2][j2])
    return smaller + larger


if __name__ == "__main__":
    A = [
        [1, 3, 7, 10, 15, 20],
        [2, 6, 9, 14, 22, 25],
        [3, 8, 10, 15, 25, 30],
        [10, 11, 12, 23, 30, 35],
        [20, 25, 30, 35, 40, 45],
    ]
    print(solve(A, 1, 1, 3, 3))
