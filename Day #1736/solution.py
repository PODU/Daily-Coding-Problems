# Day 1736: FIFO queue via two stacks (in/out); dequeue moves in->out when out empty. Amortized O(1) per op, O(n) space.

class MyQueue:
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


def main():
    q = MyQueue()
    q.enqueue(1); q.enqueue(2); q.enqueue(3)
    print(q.dequeue())
    q.enqueue(4)
    print(q.dequeue())
    print(q.dequeue())
    print(q.dequeue())


if __name__ == "__main__":
    main()
