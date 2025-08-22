# Day 150: K nearest points: max-heap of size k keyed by negated squared distance. Time O(n log k), Space O(k).
import heapq


def k_nearest(pts, c, k):
    heap = []  # (-dist2, index)
    for i, (x, y) in enumerate(pts):
        d2 = (x - c[0]) ** 2 + (y - c[1]) ** 2
        heapq.heappush(heap, (-d2, i))
        if len(heap) > k:
            heapq.heappop(heap)
    idx = sorted(i for _, i in heap)  # keep original order for stable output
    return [pts[i] for i in idx]


if __name__ == "__main__":
    pts = [(0, 0), (5, 4), (3, 1)]
    c = (1, 2)
    k = 2
    res = k_nearest(pts, c, k)
    print("[" + ", ".join("({}, {})".format(x, y) for x, y in res) + "]")
