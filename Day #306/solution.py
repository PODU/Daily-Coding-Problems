# Day 306: Sort a k-sorted array with a min-heap of size k+1. O(N log k).
import heapq


def sort_k(arr, k):
    heap = arr[:k + 1]
    heapq.heapify(heap)
    res = []
    for i in range(k + 1, len(arr)):
        res.append(heapq.heappushpop(heap, arr[i]))
    while heap:
        res.append(heapq.heappop(heap))
    return res


if __name__ == "__main__":
    arr = [6, 5, 3, 2, 8, 10, 9]
    print(" ".join(map(str, sort_k(arr, 3))))  # 2 3 5 6 8 9 10
