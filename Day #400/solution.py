# Day 400: Prefix-sum array P (P[k]=sum of first k elems); sum(i,j)=P[j]-P[i]. Preprocess O(n), query O(1).
class RangeSum:
    def __init__(self, L):
        self.P = [0] * (len(L) + 1)
        for k, x in enumerate(L):
            self.P[k + 1] = self.P[k] + x

    def sum(self, i, j):
        return self.P[j] - self.P[i]


if __name__ == "__main__":
    L = [1, 2, 3, 4, 5]
    rs = RangeSum(L)
    print(rs.sum(1, 3))
