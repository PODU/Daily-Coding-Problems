# Day 1726: Sort a k-sorted array (each element <= k from its sorted position).
# Min-heap of size k+1: pop the min as the next sorted element. O(N log k) time, O(k) space.
import heapq


def sort_k_sorted(arr, k):
    heap = arr[:k + 1]
    heapq.heapify(heap)
    result = []
    for i in range(k + 1, len(arr)):
        result.append(heapq.heappushpop(heap, arr[i]))
    while heap:
        result.append(heapq.heappop(heap))
    return result


if __name__ == "__main__":
    arr = [2, 1, 4, 3, 6, 5]  # k-sorted with k = 2
    print(" ".join(map(str, sort_k_sorted(arr, 2))))
