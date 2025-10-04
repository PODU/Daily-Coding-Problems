# Day 365: "Quack" deque (push/pop left, pull right) from three stacks.
# l holds the left part (left end on top), r the right part (right end on top);
# tmp is a transient helper used only to re-split when one side empties.
# Rebalance moves k after k ops -> O(1) amortized, O(1) extra memory.


class Quack:
    def __init__(self):
        self.l, self.r, self.tmp = [], [], []

    def push(self, x):
        self.l.append(x)

    def _rebalance(self, to, frm, to_count):
        n = len(frm)
        for _ in range(n - to_count):
            self.tmp.append(frm.pop())
        for _ in range(to_count):
            to.append(frm.pop())
        while self.tmp:
            frm.append(self.tmp.pop())

    def pop(self):
        if not self.l:
            self._rebalance(self.l, self.r, (len(self.r) + 1) // 2)
        return self.l.pop()

    def pull(self):
        if not self.r:
            self._rebalance(self.r, self.l, (len(self.l) + 1) // 2)
        return self.r.pop()


if __name__ == "__main__":
    q = Quack()
    q.push(1); q.push(2); q.push(3)
    print(q.pop())    # 3
    print(q.pull())   # 1
    q.push(4); q.push(5)
    print(q.pull())   # 2
    print(q.pop())    # 5
    print(q.pop())    # 4
