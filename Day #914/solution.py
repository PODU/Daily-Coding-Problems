# Day 914: Queue via fixed-length array blocks (capacity 4). Deque of blocks; enqueue to tail, dequeue from head.
# Amortized O(1) per op; O(n) space.
from collections import deque


class BlockQueue:
    CAP = 4

    def __init__(self):
        self.blocks = deque()
        self.head_idx = 0
        self.tail_count = 0
        self.size = 0

    def enqueue(self, v):
        if not self.blocks or self.tail_count == self.CAP:
            self.blocks.append([0] * self.CAP)
            self.tail_count = 0
        self.blocks[-1][self.tail_count] = v
        self.tail_count += 1
        self.size += 1

    def dequeue(self):
        if self.size == 0:
            raise IndexError("empty")
        v = self.blocks[0][self.head_idx]
        self.head_idx += 1
        self.size -= 1
        if self.head_idx == self.CAP or (len(self.blocks) == 1 and self.head_idx == self.tail_count):
            self.blocks.popleft()
            self.head_idx = 0
            if not self.blocks:
                self.tail_count = 0
        return v

    def get_size(self):
        return self.size


if __name__ == "__main__":
    q = BlockQueue()
    for i in range(1, 11):
        q.enqueue(i)
    dq = [q.dequeue() for _ in range(3)]
    print("Dequeued: " + " ".join(map(str, dq)))
    print("Size: " + str(q.get_size()))
