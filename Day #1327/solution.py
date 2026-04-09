# Day 1327: Queue backed by a list of fixed-length blocks (chunks).
# enqueue appends to the tail block (new block when full); dequeue pops from the head block. Amortized O(1).
from collections import deque


class BlockQueue:
    BLOCK = 4

    def __init__(self):
        self.blocks = deque()
        self.head = 0   # read index in the front block
        self.size = 0

    def enqueue(self, x):
        if not self.blocks or len(self.blocks[-1]) == self.BLOCK:
            self.blocks.append([])
        self.blocks[-1].append(x)
        self.size += 1

    def dequeue(self):
        if self.size == 0:
            raise IndexError("empty")
        x = self.blocks[0][self.head]
        self.head += 1
        self.size -= 1
        if self.head == len(self.blocks[0]):
            self.blocks.popleft()
            self.head = 0
        return x

    def get_size(self):
        return self.size


if __name__ == "__main__":
    q = BlockQueue()
    for i in range(1, 6):
        q.enqueue(i)
    print(q.dequeue())   # 1
    print(q.dequeue())   # 2
    print(q.get_size())  # 3
