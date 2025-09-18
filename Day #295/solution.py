# Day 295: Kth row of Pascal's triangle (1-indexed) via iterative binomials in one array. O(k) space, O(k) time.
def pascal_row(k):
    n = k - 1  # 0-indexed row number
    row = [1] * k
    for r in range(1, k):
        row[r] = row[r-1] * (n - r + 1) // r
    return row


if __name__ == "__main__":
    print(pascal_row(5))
