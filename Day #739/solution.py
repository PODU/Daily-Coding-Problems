# Day 739: Quack (push/pop left, pull right) via three stacks.
# On underflow of one side, rebalance by moving half from the other side using a temp stack.
# Amortized O(1) per operation, O(1) extra memory beyond the three stacks.

class Quack:
    def __init__(self):
        self.front = []  # front[-1] = leftmost
        self.back = []   # back[-1] = rightmost
        self.temp = []

    def push(self, x):
        self.front.append(x)

    def _refill_front(self):
        n = len(self.back)
        left_count = (n + 1) // 2
        right_count = n - left_count
        for _ in range(right_count):
            self.temp.append(self.back.pop())
        for _ in range(left_count):
            self.front.append(self.back.pop())
        while self.temp:
            self.back.append(self.temp.pop())

    def _refill_back(self):
        n = len(self.front)
        right_count = (n + 1) // 2
        left_count = n - right_count
        for _ in range(left_count):
            self.temp.append(self.front.pop())
        for _ in range(right_count):
            self.back.append(self.front.pop())
        while self.temp:
            self.front.append(self.temp.pop())

    def pop(self):
        if not self.front:
            self._refill_front()
        if not self.front:
            raise IndexError("empty")
        return self.front.pop()

    def pull(self):
        if not self.back:
            self._refill_back()
        if not self.back:
            raise IndexError("empty")
        return self.back.pop()


if __name__ == "__main__":
    q = Quack()
    q.push(1); q.push(2); q.push(3)
    print("pop:", q.pop())    # 3
    print("pull:", q.pull())  # 1
    q.push(4)
    print("pull:", q.pull())  # 2
    print("pop:", q.pop())    # 4
