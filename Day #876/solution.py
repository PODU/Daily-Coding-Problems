# Day 876: Sort a k-sorted array with a min-heap of size k+1. Time O(N log k), Space O(k).
import heapq


def sort_k_sorted(a, k):
    heap = []
    res = []
    for x in a:
        heapq.heappush(heap, x)
        if len(heap) > k:
            res.append(heapq.heappop(heap))
    while heap:
        res.append(heapq.heappop(heap))
    return res


if __name__ == "__main__":
    a = [6, 5, 3, 2, 8, 10, 9]
    k = 3
    print(sort_k_sorted(a, k))
