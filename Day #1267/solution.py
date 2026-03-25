# Day 1267: Range sum query with preprocessing.
# Prefix-sum array: O(n) preprocess, O(1) per sum(i, j) query.
from itertools import accumulate
from typing import List


class RangeSum:
    def __init__(self, L: List[int]):
        self.prefix = [0] + list(accumulate(L))

    def sum(self, i: int, j: int) -> int:   # L[i:j]
        return self.prefix[j] - self.prefix[i]


if __name__ == "__main__":
    rs = RangeSum([1, 2, 3, 4, 5])
    print(rs.sum(1, 3))
