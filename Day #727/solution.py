# Day 727: Running median over a stream.
# Approach: Max-heap for lower half, min-heap for upper half, kept balanced.
# Time: O(log n) per element, Space: O(n).
import heapq


def print_median(m):
    print(int(m) if m == int(m) else m)


def running_median(stream):
    lo, hi = [], []  # lo: max-heap (negated), hi: min-heap
    for x in stream:
        if not lo or x <= -lo[0]:
            heapq.heappush(lo, -x)
        else:
            heapq.heappush(hi, x)
        if len(lo) > len(hi) + 1:
            heapq.heappush(hi, -heapq.heappop(lo))
        elif len(hi) > len(lo):
            heapq.heappush(lo, -heapq.heappop(hi))
        if len(lo) == len(hi):
            print_median((-lo[0] + hi[0]) / 2)
        else:
            print_median(-lo[0])


if __name__ == "__main__":
    running_median([2, 1, 5, 7, 2, 0, 5])
