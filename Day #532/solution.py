# Day 532: Count attacking bishop pairs: group by diagonals r+c and r-c, sum C(size,2).
# Time O(N), Space O(N).
from collections import Counter


def count_attacking_pairs(bishops):
    diag1 = Counter(r + c for r, c in bishops)
    diag2 = Counter(r - c for r, c in bishops)
    pairs = 0
    for cnt in list(diag1.values()) + list(diag2.values()):
        pairs += cnt * (cnt - 1) // 2
    return pairs


if __name__ == "__main__":
    bishops = [(0, 0), (1, 2), (2, 2), (4, 0)]
    print(count_attacking_pairs(bishops))
