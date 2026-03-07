# Day 1169: K nearest points: max-heap of size k by squared Euclidean distance,
# then sort the k results by (dist, original index) for deterministic output.
# Time: O(n log k), Space: O(k).
import heapq


def k_nearest(points, central, k):
    cx, cy = central

    def dist2(p):
        return (p[0] - cx) ** 2 + (p[1] - cy) ** 2

    heap = []  # max-heap via negated distance, keeps k smallest
    for i, p in enumerate(points):
        heapq.heappush(heap, (-dist2(p), -i))
        if len(heap) > k:
            heapq.heappop(heap)

    idx = [-neg_i for _, neg_i in heap]
    idx.sort(key=lambda i: (dist2(points[i]), i))
    return [points[i] for i in idx]


if __name__ == "__main__":
    points = [(0, 0), (5, 4), (3, 1)]
    central = (1, 2)
    k = 2
    print(k_nearest(points, central, k))
