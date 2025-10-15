# Day 438: Stack via a max-heap. Each push tags the item with an increasing
# counter; the heap's max counter is the most-recent item. push/pop O(log n).
import heapq


class StackFromHeap:
    def __init__(self):
        self.heap = []        # stores (-counter, item) -> min-heap acts as max-heap
        self.counter = 0

    def push(self, item):
        heapq.heappush(self.heap, (-self.counter, item))
        self.counter += 1

    def pop(self):
        if not self.heap:
            raise IndexError("stack is empty")
        return heapq.heappop(self.heap)[1]


if __name__ == "__main__":
    s = StackFromHeap()
    s.push(1); s.push(2); s.push(3)
    print(s.pop())  # 3
    print(s.pop())  # 2
    s.push(4)
    print(s.pop())  # 4
    print(s.pop())  # 1
