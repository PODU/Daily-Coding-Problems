# Day 429: kth row of Pascal's triangle (0-indexed: row 0 = [1]).
# Multiplicative recurrence row[j] = row[j-1]*(k-j+1)/j, in place. Time O(k), Space O(k).
def pascal_row(k):
    row = [1] * (k + 1)
    for j in range(1, k + 1):
        row[j] = row[j - 1] * (k - j + 1) // j
    return row


if __name__ == "__main__":
    print(" ".join(map(str, pascal_row(4))))
