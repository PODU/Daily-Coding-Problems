# Day 377: Sliding window median via two heaps with lazy deletion.
# Time: O(n log k), Space: O(k).
import heapq


class DualHeap:
    def __init__(self):
        self.small = []   # max-heap (store negatives)
        self.large = []   # min-heap
        self.delayed = {}
        self.ss = 0
        self.ls = 0

    def _prune(self, heap):
        sign = -1 if heap is self.small else 1
        while heap:
            num = sign * heap[0]
            if self.delayed.get(num, 0) > 0:
                self.delayed[num] -= 1
                if self.delayed[num] == 0:
                    del self.delayed[num]
                heapq.heappop(heap)
            else:
                break

    def _balance(self):
        if self.ss > self.ls + 1:
            heapq.heappush(self.large, -self.small[0]); heapq.heappop(self.small)
            self.ss -= 1; self.ls += 1; self._prune(self.small)
        elif self.ss < self.ls:
            heapq.heappush(self.small, -self.large[0]); heapq.heappop(self.large)
            self.ls -= 1; self.ss += 1; self._prune(self.large)

    def insert(self, num):
        if not self.small or num <= -self.small[0]:
            heapq.heappush(self.small, -num); self.ss += 1
        else:
            heapq.heappush(self.large, num); self.ls += 1
        self._balance()

    def erase(self, num):
        self.delayed[num] = self.delayed.get(num, 0) + 1
        if num <= -self.small[0]:
            self.ss -= 1
            if num == -self.small[0]:
                self._prune(self.small)
        else:
            self.ls -= 1
            if num == self.large[0]:
                self._prune(self.large)
        self._balance()

    def median(self, k):
        if k % 2 == 1:
            return float(-self.small[0])
        return (-self.small[0] + self.large[0]) / 2.0


def fmt(d):
    return str(int(d)) if d == int(d) else str(d)


if __name__ == "__main__":
    arr = [-1, 5, 13, 8, 2, 3, 3, 1]
    k = 3
    dh = DualHeap()
    for i in range(k):
        dh.insert(arr[i])
    n = len(arr)
    for i in range(k, n + 1):
        win = arr[i - k:i]
        print(f"{fmt(dh.median(k))} <- median of {win}")
        if i < n:
            dh.insert(arr[i]); dh.erase(arr[i - k])
