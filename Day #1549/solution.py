# Day 1549: Toeplitz check: every element equals its top-left neighbor matrix[i-1][j-1].
# Time O(m*n), Space O(1).

def is_toeplitz(m):
    for i in range(1, len(m)):
        for j in range(1, len(m[i])):
            if m[i][j] != m[i - 1][j - 1]:
                return False
    return True


if __name__ == "__main__":
    m = [
        [1, 2, 3, 4, 8],
        [5, 1, 2, 3, 4],
        [4, 5, 1, 2, 3],
        [7, 4, 5, 1, 2],
    ]
    print("true" if is_toeplitz(m) else "false")
