# Day 356: Queue built from a deque of fixed-capacity blocks (cap 4). Track head/tail block+offset and an O(1) size.
# enqueue/dequeue/get_size all amortized O(1) time; O(n) space.
from collections import deque


class BlockQueue:
    CAP = 4

    def __init__(self):
        self.blocks = deque()
        self.head_off = 0  # offset within front block
        self.tail_off = 0  # next write offset within back block
        self.sz = 0

    def enqueue(self, v):
        if not self.blocks or self.tail_off == self.CAP:
            self.blocks.append([None] * self.CAP)
            self.tail_off = 0
        self.blocks[-1][self.tail_off] = v
        self.tail_off += 1
        self.sz += 1

    def dequeue(self):
        v = self.blocks[0][self.head_off]
        self.head_off += 1
        self.sz -= 1
        if self.head_off == self.CAP:  # front block fully consumed
            self.blocks.popleft()
            self.head_off = 0
        return v

    def get_size(self):
        return self.sz


def main():
    q = BlockQueue()
    for v in [1, 2, 3, 4, 5]:
        q.enqueue(v)
    print("size={}".format(q.get_size()))
    print(q.dequeue())
    print(q.dequeue())
    print(q.dequeue())
    print("size={}".format(q.get_size()))


if __name__ == "__main__":
    main()
