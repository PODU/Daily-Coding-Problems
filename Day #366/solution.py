# Day 366: Rearrange so no two adjacent chars match (else null).
# Greedy with a max-heap by frequency; always place the most frequent char that
# isn't the one just placed. Feasible iff maxFreq <= (n+1)//2. Time O(n log k).
import heapq
from collections import Counter


def reorganize(s):
    heap = [(-c, ch) for ch, c in Counter(s).items()]
    heapq.heapify(heap)
    res = []
    prev = None  # (count, char) waiting to be re-added
    while heap:
        c, ch = heapq.heappop(heap)
        res.append(ch)
        if prev and prev[0] < 0:
            heapq.heappush(heap, prev)
        prev = (c + 1, ch)  # used one occurrence
    return "".join(res) if len(res) == len(s) else None


if __name__ == "__main__":
    print(reorganize("yyz"))   # yzy
    print(reorganize("yyy"))   # None
