# Day 491: Running median of a stream.
# Two heaps: a max-heap (negated) keeps the lower half, a min-heap the upper half;
# rebalance so the lower half has equal size or one extra, so the median is its top.
# Time O(log n) per element, Space O(n).
import heapq


def running_median(stream):
    lo, hi = [], []  # lo: max-heap via negation, hi: min-heap
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
        if len(lo) == len(hi):
            m = (-lo[0] + hi[0]) / 2
            out.append(int(m) if m == int(m) else m)
        else:
            out.append(-lo[0])
    return out


if __name__ == "__main__":
    for m in running_median([2, 1, 5, 7, 2, 0, 5]):
        print(m)  # 2, 1.5, 2, 3.5, 2, 2, 2
