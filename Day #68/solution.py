# Day 68: Count attacking bishop pairs: group by diagonals d1=row+col, d2=row-col; sum k*(k-1)/2. Time O(N), Space O(N).
from collections import Counter


def count_attacking_pairs(bishops):
    d1 = Counter(r + c for r, c in bishops)
    d2 = Counter(r - c for r, c in bishops)
    return sum(k * (k - 1) // 2 for k in d1.values()) + \
           sum(k * (k - 1) // 2 for k in d2.values())


if __name__ == "__main__":
    bishops = [(0, 0), (1, 2), (2, 2), (4, 0)]
    print(count_attacking_pairs(bishops))
