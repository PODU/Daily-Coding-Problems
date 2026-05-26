# Day 1564: Staircase count from top-right in O(N+M): smaller = count(<low); larger = N*M - count(<high). Time O(N+M), Space O(1).


def count_less(M, x):
    # Number of elements strictly less than x in a row/col sorted matrix.
    n, m = len(M), len(M[0])
    cnt = 0
    r, c = 0, m - 1
    while r < n and c >= 0:
        if M[r][c] < x:
            cnt += c + 1
            r += 1
        else:
            c -= 1
    return cnt


def main():
    M = [
        [1, 3, 7, 10, 15, 20],
        [2, 6, 9, 14, 22, 25],
        [3, 8, 10, 15, 25, 30],
        [10, 11, 12, 23, 30, 35],
        [20, 25, 30, 35, 40, 45],
    ]
    low = M[1][1]   # 6
    high = M[3][3]  # 23
    total = len(M) * len(M[0])
    smaller = count_less(M, low)          # elements < 6
    larger = total - count_less(M, high)  # elements >= 23
    print(smaller + larger)


if __name__ == "__main__":
    main()
