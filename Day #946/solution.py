# Day 946: kth row of Pascal's triangle (1-indexed) using O(k) space.
# In-place update of a single row, right-to-left. Time O(k^2), Space O(k).

def pascal_row(k):
    row = [1]
    for _ in range(1, k):
        row.append(0)
        for j in range(len(row) - 1, 0, -1):
            row[j] += row[j - 1]
    return row


if __name__ == "__main__":
    k = 5  # README example -> 5th row
    print(" ".join(map(str, pascal_row(k))))
