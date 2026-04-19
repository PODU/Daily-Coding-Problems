# Day 1388: FIFO queue via two stacks (in/out); amortized O(1) per op, O(n) space.

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
    print(q.dequeue())  # 1
    q.enqueue(4)
    print(q.dequeue())  # 2
    print(q.dequeue())  # 3
    print(q.dequeue())  # 4


if __name__ == "__main__":
    main()
