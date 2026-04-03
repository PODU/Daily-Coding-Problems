# Day 1292: kth (0-indexed) row of Pascal's triangle.
# Update row in place from right to left. O(k^2) time, O(k) space.


def pascal_row(k: int):
    row = [1] * (k + 1)
    for i in range(1, k + 1):
        for j in range(i - 1, 0, -1):
            row[j] += row[j - 1]
    return row


if __name__ == "__main__":
    print(" ".join(map(str, pascal_row(4))))  # 1 4 6 4 1
