# Day 650: Per-row binary search: count elements < A[i1][j1] (bisect_left) plus elements >= A[i2][j2] (M - bisect_left).
# Upper bound taken inclusive (>=) to match reference example. Time O(N log M), space O(1).
from bisect import bisect_left


def main():
    A = [
        [1, 3, 7, 10, 15, 20],
        [2, 6, 9, 14, 22, 25],
        [3, 8, 10, 15, 25, 30],
        [10, 11, 12, 23, 30, 35],
        [20, 25, 30, 35, 40, 45],
    ]
    i1, j1, i2, j2 = 1, 1, 3, 3
    pivot1 = A[i1][j1]  # 6
    pivot2 = A[i2][j2]  # 23
    M = len(A[0])
    count_smaller = 0
    count_upper = 0
    for row in A:
        count_smaller += bisect_left(row, pivot1)        # strictly less than pivot1
        count_upper += M - bisect_left(row, pivot2)      # >= pivot2 (inclusive)
    print(count_smaller + count_upper)


if __name__ == "__main__":
    main()
