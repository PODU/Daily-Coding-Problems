# Day 898: Stack via max-heap: tag each push with increasing priority; heap pops highest priority (most recent). O(log n)/op, O(n) space.
import heapq


class HeapStack:
    def __init__(self):
        self._heap = []  # entries are (-priority, value) for max behavior via min-heap
        self._counter = 0

    def push(self, v):
        heapq.heappush(self._heap, (-self._counter, v))
        self._counter += 1

    def pop(self):
        if not self._heap:
            raise IndexError("pop from empty stack")
        return heapq.heappop(self._heap)[1]


if __name__ == "__main__":
    s = HeapStack()
    s.push(1); s.push(2); s.push(3)
    print(s.pop())  # 3
    print(s.pop())  # 2
    s.push(4)
    print(s.pop())  # 4
    print(s.pop())  # 1
