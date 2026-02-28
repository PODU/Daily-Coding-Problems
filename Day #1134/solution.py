# Day 1134: Running median with two heaps (max-heap low via negation, min-heap high). O(log n) per insert.
import heapq


def fmt(x):
    # print whole numbers without decimal, halves as .5
    if x == int(x):
        return str(int(x))
    return str(x)


def running_median(nums):
    low = []   # max-heap (negated)
    high = []  # min-heap
    out = []
    for x in nums:
        if not low or x <= -low[0]:
            heapq.heappush(low, -x)
        else:
            heapq.heappush(high, x)
        if len(low) > len(high) + 1:
            heapq.heappush(high, -heapq.heappop(low))
        elif len(high) > len(low):
            heapq.heappush(low, -heapq.heappop(high))

        if len(low) == len(high):
            med = (-low[0] + high[0]) / 2
        else:
            med = -low[0]
        out.append(fmt(med))
    return out


if __name__ == "__main__":
    for line in running_median([2, 1, 5, 7, 2, 0, 5]):
        print(line)
