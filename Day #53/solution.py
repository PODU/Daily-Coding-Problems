# Day 53: FIFO queue from two stacks. Amortized O(1) per op.
# in-stack receives pushes; out-stack serves pops (refilled when empty).


class Queue:
    def __init__(self):
        self._in = []
        self._out = []

    def enqueue(self, x):
        self._in.append(x)

    def dequeue(self):
        if not self._out:
            while self._in:
                self._out.append(self._in.pop())
        return self._out.pop()


if __name__ == "__main__":
    q = Queue()
    q.enqueue(1)
    q.enqueue(2)
    q.enqueue(3)
    print(q.dequeue())  # 1
    print(q.dequeue())  # 2
    q.enqueue(4)
    print(q.dequeue())  # 3
    print(q.dequeue())  # 4
