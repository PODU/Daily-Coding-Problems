# Day 154: Stack using only a max-heap. Tag each item with an increasing
# priority so the heap always pops the most recent. push/pop O(log n).
import heapq


class HeapStack:
    def __init__(self):
        self._heap = []  # entries of (-priority, value) for max behavior
        self._counter = 0

    def push(self, item):
        heapq.heappush(self._heap, (-self._counter, item))
        self._counter += 1

    def pop(self):
        if not self._heap:
            raise IndexError("pop from empty stack")
        return heapq.heappop(self._heap)[1]

    def empty(self):
        return not self._heap


if __name__ == "__main__":
    s = HeapStack()
    s.push(1)
    s.push(2)
    s.push(3)
    print(s.pop())  # 3
    print(s.pop())  # 2
    s.push(4)
    print(s.pop())  # 4
    print(s.pop())  # 1
