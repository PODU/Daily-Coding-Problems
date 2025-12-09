# Day 722: Minimum swaps so each couple sits side by side.
# Approach: Greedy - fix each even seat; partner = x^1. Each swap places one couple.
# Answer equals N - (#cycles). Time: O(N), Space: O(N).

def min_swaps(row):
    row = row[:]
    pos = {v: i for i, v in enumerate(row)}
    swaps = 0
    for i in range(0, len(row), 2):
        partner = row[i] ^ 1
        if row[i + 1] != partner:
            j = pos[partner]
            pos[row[i + 1]], pos[partner] = j, i + 1
            row[i + 1], row[j] = row[j], row[i + 1]
            swaps += 1
    return swaps


if __name__ == "__main__":
    print("Minimum swaps:", min_swaps([0, 2, 1, 3]))  # 1
