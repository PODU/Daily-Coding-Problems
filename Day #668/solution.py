# Day 668: Toeplitz matrix check. Every cell equals its top-left neighbor. Time O(m*n), Space O(1).


def is_toeplitz(m):
    return all(
        m[i][j] == m[i - 1][j - 1]
        for i in range(1, len(m))
        for j in range(1, len(m[i]))
    )


if __name__ == "__main__":
    m = [
        [1, 2, 3, 4, 8],
        [5, 1, 2, 3, 4],
        [4, 5, 1, 2, 3],
        [7, 4, 5, 1, 2],
    ]
    print(is_toeplitz(m))  # True
