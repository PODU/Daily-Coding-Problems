# Day 1442: Implement a stack using only a heap.
# Approach: tag each item with an increasing counter as its key so the most
# recently pushed item is always the heap's max. push/pop O(log n).
import heapq


class Stack:
    def __init__(self):
        self._heap = []          # python heapq is a min-heap...
        self._counter = 0

    def push(self, item):
        # negate counter to simulate a max-heap (largest counter = newest)
        heapq.heappush(self._heap, (-self._counter, item))
        self._counter += 1

    def pop(self):
        if not self._heap:
            raise IndexError("pop from empty stack")
        return heapq.heappop(self._heap)[1]


if __name__ == "__main__":
    s = Stack()
    s.push(1); s.push(2); s.push(3)
    print(s.pop(), s.pop(), s.pop())  # 3 2 1
