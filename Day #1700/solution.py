# Day 1700: Deque ("quack") from three stacks: L (top=leftmost), R (top=rightmost), T temp.
# On empty side, move half the other stack across (T as transient buffer) => O(1) amortized.
class Quack:
    def __init__(self):
        self.L = []  # top = leftmost
        self.R = []  # top = rightmost
        self.T = []  # transient buffer

    def push(self, x):  # add to LEFT end
        self.L.append(x)

    def pop(self):  # remove LEFT end
        if not self.L:
            self._rebalance_L_from_R()
        return self.L.pop()

    def pull(self):  # remove RIGHT end
        if not self.R:
            self._rebalance_R_from_L()
        return self.R.pop()

    def _rebalance_L_from_R(self):
        size = len(self.R)
        half = size // 2 or 1
        keep = size - half
        for _ in range(keep):
            self.T.append(self.R.pop())
        while self.R:
            self.L.append(self.R.pop())
        while self.T:
            self.R.append(self.T.pop())

    def _rebalance_R_from_L(self):
        size = len(self.L)
        half = size // 2 or 1
        keep = size - half
        for _ in range(keep):
            self.T.append(self.L.pop())
        while self.L:
            self.R.append(self.L.pop())
        while self.T:
            self.L.append(self.T.pop())


if __name__ == "__main__":
    q = Quack()
    q.push(1); q.push(2); q.push(3)  # left-to-right: 3,2,1
    print(q.pop())   # 3
    print(q.pull())  # 1
    q.push(4)        # now 4,2
    print(q.pop())   # 4
    print(q.pull())  # 2
