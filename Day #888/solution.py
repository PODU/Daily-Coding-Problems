# Day 888: Nearest k points: max-heap of size k by squared distance. Time O(n log k), Space O(k).
import heapq


def nearest_k(points, central, k):
    heap = []  # max-heap via negated squared distance
    for x, y in points:
        d2 = (x - central[0]) ** 2 + (y - central[1]) ** 2
        heapq.heappush(heap, (-d2, x, y))
        if len(heap) > k:
            heapq.heappop(heap)
    res = sorted((x, y) for _, x, y in heap)
    return res


if __name__ == "__main__":
    points = [(0, 0), (5, 4), (3, 1)]
    central = (1, 2)
    res = nearest_k(points, central, 2)
    print("[" + ", ".join("({}, {})".format(x, y) for x, y in res) + "]")
