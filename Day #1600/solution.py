# Day 1600: Stack via a single MAX-heap keyed by a monotonic counter; newest key is largest so pops first (LIFO).
# push/pop O(log n), space O(n). Python heapq is a min-heap, so negate the counter.
import heapq


class StackViaHeap:
    def __init__(self):
        self._heap = []      # stores (-counter, value)
        self._counter = 0

    def push(self, x):
        heapq.heappush(self._heap, (-self._counter, x))
        self._counter += 1

    def pop(self):
        if not self._heap:
            raise IndexError("pop from empty stack")
        return heapq.heappop(self._heap)[1]

    def empty(self):
        return not self._heap


def main():
    s = StackViaHeap()
    s.push(1); s.push(2); s.push(3)
    out = []
    out.append(s.pop())  # 3
    out.append(s.pop())  # 2
    s.push(4)
    out.append(s.pop())  # 4
    out.append(s.pop())  # 1
    print(" ".join(map(str, out)))


if __name__ == "__main__":
    main()
