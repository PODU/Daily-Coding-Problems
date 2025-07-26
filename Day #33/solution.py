# Day 33: Running median with two heaps (max-heap lower half, min-heap upper half). O(log n) per insert.
import heapq


def running_median(stream):
    lo = []  # max-heap via negation (lower half)
    hi = []  # min-heap (upper half)
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
        else:
            m = float(-lo[0])
        out.append(str(int(m)) if m.is_integer() else str(m))
    return out


if __name__ == "__main__":
    print("\n".join(running_median([2, 1, 5, 7, 2, 0, 5])))
