# Day 443: FIFO queue from two stacks. Amortized O(1) per op: push onto `in_`,
# pop from `out`, refilling `out` from `in_` when empty.


class QueueTwoStacks:
    def __init__(self):
        self.in_ = []
        self.out = []

    def enqueue(self, x):
        self.in_.append(x)

    def dequeue(self):
        if not self.out:
            if not self.in_:
                raise IndexError("queue is empty")
            while self.in_:
                self.out.append(self.in_.pop())
        return self.out.pop()


if __name__ == "__main__":
    q = QueueTwoStacks()
    q.enqueue(1); q.enqueue(2); q.enqueue(3)
    print(q.dequeue())  # 1
    print(q.dequeue())  # 2
    q.enqueue(4)
    print(q.dequeue())  # 3
    print(q.dequeue())  # 4
