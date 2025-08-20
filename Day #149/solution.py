# Day 149: Range sum via prefix sums: O(n) preprocessing, O(1) per query. sum(i,j) = pre[j]-pre[i].

class RangeSum:
    def __init__(self, L):
        self.pre = [0] * (len(L) + 1)
        for k, v in enumerate(L):
            self.pre[k + 1] = self.pre[k] + v

    def sum(self, i, j):
        return self.pre[j] - self.pre[i]


if __name__ == "__main__":
    L = [1, 2, 3, 4, 5]
    rs = RangeSum(L)
    print(rs.sum(1, 3))
