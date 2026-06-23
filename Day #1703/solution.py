# Day 1703: Prefix sums: precompute prefix[k]=sum(L[0:k]); sum(i,j)=prefix[j]-prefix[i].
# Preprocess O(n), query O(1), Space O(n).
class RangeSum:
    def __init__(self, L):
        self.prefix = [0] * (len(L) + 1)
        for k, x in enumerate(L):
            self.prefix[k + 1] = self.prefix[k] + x

    def sum(self, i, j):
        return self.prefix[j] - self.prefix[i]

def main():
    L = [1, 2, 3, 4, 5]
    rs = RangeSum(L)
    print(rs.sum(1, 3))

if __name__ == "__main__":
    main()
