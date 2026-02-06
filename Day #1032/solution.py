# Day 1032: Check if a matrix is Toeplitz.
# Compare each element to its top-left neighbor: m[i][j] == m[i-1][j-1]. O(rows*cols) time, O(1) space.


def is_toeplitz(m):
    return all(
        m[i][j] == m[i - 1][j - 1]
        for i in range(1, len(m))
        for j in range(1, len(m[i]))
    )


if __name__ == "__main__":
    matrix = [
        [1, 2, 3, 4, 8],
        [5, 1, 2, 3, 4],
        [4, 5, 1, 2, 3],
        [7, 4, 5, 1, 2],
    ]
    print(is_toeplitz(matrix))
