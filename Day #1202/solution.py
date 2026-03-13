# Day 1202: Minimum swaps so couples sit side by side.
# Greedy: for each even seat, swap partner of its occupant into the next seat. Time O(N), Space O(N).


def min_swaps(row):
    row = row[:]
    pos = {v: i for i, v in enumerate(row)}
    swaps = 0
    for i in range(0, len(row), 2):
        partner = row[i] ^ 1  # couples are (0,1),(2,3),...
        if row[i + 1] != partner:
            j = pos[partner]
            pos[row[i + 1]], pos[row[j]] = pos[row[j]], pos[row[i + 1]]
            row[i + 1], row[j] = row[j], row[i + 1]
            swaps += 1
    return swaps


if __name__ == "__main__":
    print(min_swaps([0, 2, 1, 3]))  # 1
