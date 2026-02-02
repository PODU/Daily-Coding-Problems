# Day 1004: Range sum query sum(i, j) = L[i:j].
# Pre-process a prefix-sum array (O(N)); each query is prefix[j] - prefix[i] in O(1).

class RangeSum:
    def __init__(self, L):
        self.prefix = [0] * (len(L) + 1)
        for idx, v in enumerate(L):
            self.prefix[idx + 1] = self.prefix[idx] + v

    def sum(self, i, j):
        return self.prefix[j] - self.prefix[i]


if __name__ == "__main__":
    rs = RangeSum([1, 2, 3, 4, 5])
    print(rs.sum(1, 3))  # 5  (sum of [2, 3])
