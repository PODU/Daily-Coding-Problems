# Day 488: Queue backed by a set of fixed-length arrays (blocks).
# Blocks of size B chained; head/tail indices roll over to next block.
# enqueue/dequeue amortized O(1), get_size O(1). Space O(n).
from collections import deque


class BlockQueue:
    B = 4  # fixed block length

    def __init__(self):
        self.blocks = deque()  # set of fixed-length arrays
        self.head = 0
        self.tail = 0
        self.sz = 0

    def enqueue(self, x):
        if not self.blocks or self.tail == self.B:  # allocate a new fixed block
            self.blocks.append([0] * self.B)
            self.tail = 0
        self.blocks[-1][self.tail] = x
        self.tail += 1
        self.sz += 1

    def dequeue(self):
        if self.sz == 0:
            raise IndexError("empty")
        x = self.blocks[0][self.head]
        self.head += 1
        self.sz -= 1
        if self.head == self.B:        # front block exhausted -> free it
            self.blocks.popleft()
            self.head = 0
        if len(self.blocks) == 1 and self.head == self.tail:  # single block consumed
            self.blocks.clear()
            self.head = self.tail = 0
        return x

    def get_size(self):
        return self.sz


if __name__ == "__main__":
    q = BlockQueue()
    for i in range(1, 7):
        q.enqueue(i)  # 1..6
    print("size=%d" % q.get_size())  # 6
    print("deq=%d" % q.dequeue())    # 1
    print("deq=%d" % q.dequeue())    # 2
    q.enqueue(7)
    q.enqueue(8)
    print("size=%d" % q.get_size())  # 6
    out = []
    while q.get_size() > 0:
        out.append(str(q.dequeue()))  # 3 4 5 6 7 8
    print(" ".join(out))
