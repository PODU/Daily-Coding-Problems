# Day 1116: Day 1116 - Quack: push/pop left, pull right, using three stacks.
# Approach: two stacks L (top=leftmost) and R (top=rightmost); when one empties,
# rebalance by splitting the other in half using the third stack T as scratch.
# Amortized O(1) per op, O(1) extra memory beyond the three stacks.

class Quack:
    def __init__(self):
        self.L = []  # left stack, top = leftmost element
        self.R = []  # right stack, top = rightmost element
        self.T = []  # scratch (empty between operations)

    def push(self, x):
        self.L.append(x)

    def _rebalance_to_L(self):
        # L empty: split R, give left half to L, keep right half in R
        m = len(self.R)
        right_count = m // 2
        for _ in range(right_count):
            self.T.append(self.R.pop())
        while self.R:
            self.L.append(self.R.pop())
        while self.T:
            self.R.append(self.T.pop())

    def _rebalance_to_R(self):
        m = len(self.L)
        left_count = m // 2
        for _ in range(left_count):
            self.T.append(self.L.pop())
        while self.L:
            self.R.append(self.L.pop())
        while self.T:
            self.L.append(self.T.pop())

    def pop(self):
        if not self.L:
            self._rebalance_to_L()
        if not self.L:
            raise IndexError("pop from empty quack")
        return self.L.pop()

    def pull(self):
        if not self.R:
            self._rebalance_to_R()
        if not self.R:
            raise IndexError("pull from empty quack")
        return self.R.pop()


if __name__ == "__main__":
    q = Quack()
    for x in (1, 2, 3):
        q.push(x)            # list left->right: 3 2 1
    print("pop:", q.pop())   # 3
    print("pull:", q.pull()) # 1
    for x in (4, 5):
        q.push(x)            # 5 4 2
    print("pull:", q.pull()) # 2
    print("pop:", q.pop())   # 5
    print("pull:", q.pull()) # 4
