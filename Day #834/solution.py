# Day 834: Minimum swaps to seat N couples side by side.
# Greedy: partner of p is p^1; swap mismatched partner into place. Time: O(N), Space: O(N).


def min_swaps(row):
    row = list(row)
    n = len(row)
    pos = [0] * n
    for i, v in enumerate(row):
        pos[v] = i
    swaps = 0
    for i in range(0, n, 2):
        partner = row[i] ^ 1
        if row[i + 1] != partner:
            j = pos[partner]
            pos[row[i + 1]], pos[row[j]] = j, i + 1
            row[i + 1], row[j] = row[j], row[i + 1]
            swaps += 1
    return swaps


if __name__ == "__main__":
    print(min_swaps([0, 2, 1, 3]))
