# Day 633: Sort a k-sorted (nearly sorted) array.
# Approach: min-heap of size k+1; pop smallest as we slide the window.
# Time: O(N log k), Space: O(k).
import heapq


def sort_k_sorted(arr, k):
    heap = []
    res = []
    for x in arr:
        heapq.heappush(heap, x)
        if len(heap) > k:            # window holds at most k+1 elements
            res.append(heapq.heappop(heap))
    while heap:
        res.append(heapq.heappop(heap))
    return res


if __name__ == "__main__":
    arr = [2, 1, 4, 3, 5]  # k = 1
    print(sort_k_sorted(arr, 1))
