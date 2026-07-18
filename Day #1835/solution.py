# Day 1835: Running median with two heaps (max-heap for lower half, min-heap for upper).
# O(log n) per element.
import heapq


def running_median(stream):
    lo = []  # lower half as max-heap (store negatives)
    hi = []  # upper half as min-heap
    out = []
    for x in stream:
        if not lo or x <= -lo[0]:
            heapq.heappush(lo, -x)
        else:
            heapq.heappush(hi, x)
        if len(lo) > len(hi) + 1:
            heapq.heappush(hi, -heapq.heappop(lo))
        elif len(hi) > len(lo):
            heapq.heappush(lo, -heapq.heappop(hi))

        if len(lo) > len(hi):
            m = float(-lo[0])
        else:
            m = (-lo[0] + hi[0]) / 2.0
        out.append(int(m) if m == int(m) else m)
    return out


if __name__ == "__main__":
    for m in running_median([2, 1, 5, 7, 2, 0, 5]):
        print(m)
