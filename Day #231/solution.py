# Day 231: Reorganize string: greedily place the most frequent remaining char that differs from the last.
# Max-heap by count. Time: O(n log A), Space: O(A).
import heapq
from collections import Counter


def reorganize(s):
    heap = [(-c, ch) for ch, c in Counter(s).items()]
    heapq.heapify(heap)
    res = []
    prev = None
    while heap:
        c, ch = heapq.heappop(heap)
        res.append(ch)
        if prev and prev[0] < 0:
            heapq.heappush(heap, prev)
        prev = (c + 1, ch)
    return "".join(res) if len(res) == len(s) else None


if __name__ == "__main__":
    print(reorganize("aaabbc"))  # ababac (a valid arrangement)
    print(reorganize("aaab"))    # None
