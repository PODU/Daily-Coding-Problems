# Day 1452: Sort a k-sorted array (each element <= k from its place) using a
# min-heap of size k+1. Time O(N log k), Space O(k).
import heapq
from typing import List


def sort_k_sorted(a: List[int], k: int) -> List[int]:
    heap = []
    out = []
    for x in a:
        heapq.heappush(heap, x)
        if len(heap) > k:
            out.append(heapq.heappop(heap))
    while heap:
        out.append(heapq.heappop(heap))
    return out


if __name__ == "__main__":
    a = [2, 6, 3, 12, 56, 8]
    print(sort_k_sorted(a, 3))  # [2, 3, 6, 8, 12, 56]
