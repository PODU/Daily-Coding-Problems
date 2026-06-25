# Day 1713: Unrolled/paged queue: list of fixed-length blocks; head/tail indices. Amortized O(1) per op.
from collections import deque


class BlockQueue:
    BS = 4

    def __init__(self):
        self.blocks = deque()
        self.head = 0
        self.tail = 0
        self.sz = 0

    def enqueue(self, x):
        if not self.blocks or self.tail == self.BS:
            self.blocks.append([0] * self.BS)
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
        if self.head == self.BS or (len(self.blocks) == 1 and self.head == self.tail):
            self.blocks.popleft()
            self.head = 0
            if not self.blocks:
                self.tail = 0
        return x

    def get_size(self):
        return self.sz

    def num_blocks(self):
        return len(self.blocks)


if __name__ == "__main__":
    q = BlockQueue()
    for i in range(1, 11):
        q.enqueue(i)
    print("size after enqueue 1..10:", q.get_size())
    print("blocks allocated:", q.num_blocks())
    print("dequeue 3:", q.dequeue(), q.dequeue(), q.dequeue())
    print("size:", q.get_size())
    q.enqueue(11)
    print("enqueue 11, size:", q.get_size())
    rest = []
    while q.get_size() > 0:
        rest.append(str(q.dequeue()))
    print("dequeue rest:", " ".join(rest))
    print("size:", q.get_size())
