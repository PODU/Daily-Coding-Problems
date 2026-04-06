# Day 1311: Quack (deque) via three stacks. On underflow of one end, rebalance by moving
# half the elements through the third stack -> O(1) amortized, O(1) extra memory.

class Quack:
    def __init__(self):
        self.L = []  # left stack, top = leftmost
        self.R = []  # right stack, top = rightmost
        self.T = []  # temp

    @staticmethod
    def _rebalance(src, dst, tmp):
        n = len(src)
        k = n // 2  # elements that stay in src
        for _ in range(k):
            tmp.append(src.pop())
        for _ in range(n - k):
            dst.append(src.pop())
        for _ in range(k):
            src.append(tmp.pop())

    def push(self, x):
        self.L.append(x)

    def pop(self):
        if not self.L:
            self._rebalance(self.R, self.L, self.T)
        return self.L.pop()

    def pull(self):
        if not self.R:
            self._rebalance(self.L, self.R, self.T)
        return self.R.pop()


if __name__ == "__main__":
    q = Quack()
    q.push(1); q.push(2); q.push(3)
    print(q.pop())   # 3
    print(q.pull())  # 1
    q.push(4)
    print(q.pull())  # 2
    print(q.pop())   # 4
