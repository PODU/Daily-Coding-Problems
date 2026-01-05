# Day 858: Running median of a stream.
# Approach: max-heap (lower half) + min-heap (upper half), balanced sizes.
# Time: O(n log n) total, Space: O(n).
import heapq


def running_median(stream):
    lo = []  # max-heap via negation
    hi = []  # min-heap
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
            out.append(float(-lo[0]))
        else:
            out.append((-lo[0] + hi[0]) / 2.0)
    return out


if __name__ == "__main__":
    for m in running_median([2, 1, 5, 7, 2, 0, 5]):
        print(int(m) if m.is_integer() else m)
