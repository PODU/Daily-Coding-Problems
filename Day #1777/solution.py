# Day 1777: Count attacking bishop pairs: group by diagonal (r-c) and anti-diagonal (r+c), sum c*(c-1)/2.
# Time: O(N), Space: O(N).
from collections import Counter


def count_attacking_pairs(M, bishops):
    diag = Counter(r - c for r, c in bishops)
    anti = Counter(r + c for r, c in bishops)
    pairs = sum(v * (v - 1) // 2 for v in diag.values())
    pairs += sum(v * (v - 1) // 2 for v in anti.values())
    return pairs


if __name__ == "__main__":
    M = 5
    bishops = [(0, 0), (1, 2), (2, 2), (4, 0)]
    print(count_attacking_pairs(M, bishops))
