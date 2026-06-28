# Day 1731: kth row of Pascal's triangle (1-indexed) using O(k) space.
# Binomial coefficients built in place. Time O(k), Space O(k).

def pascal_row(k):
    row = [1] * k
    for i in range(1, k):
        row[i] = row[i - 1] * (k - i) // i
    return row


if __name__ == "__main__":
    k = 5  # row [1, 4, 6, 4, 1]
    print(" ".join(map(str, pascal_row(k))))
