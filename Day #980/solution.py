# Day 980: Count attacking bishop pairs by grouping on diagonals (row-col) and anti-diagonals (row+col).
# For each group of size k, add k*(k-1)//2. Time O(N), Space O(N).
from collections import Counter


def count_attacking_pairs(bishops):
    diag = Counter(r - c for r, c in bishops)
    anti = Counter(r + c for r, c in bishops)
    return sum(k * (k - 1) // 2 for k in diag.values()) + \
           sum(k * (k - 1) // 2 for k in anti.values())


if __name__ == "__main__":
    bishops = [(0, 0), (1, 2), (2, 2), (4, 0)]
    print(count_attacking_pairs(bishops))  # 2
