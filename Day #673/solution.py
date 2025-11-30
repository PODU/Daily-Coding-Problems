# Day 673: K nearest points to a center. heapq.nsmallest on squared distance.
# Time O(n log k), Space O(k). No sqrt needed for comparison.
import heapq


def k_nearest(pts, c, k):
    def d2(p):
        return (p[0] - c[0]) ** 2 + (p[1] - c[1]) ** 2

    # (dist, original index) keeps ties in input order
    return [p for _, _, p in heapq.nsmallest(k, ((d2(p), i, p) for i, p in enumerate(pts)))]


if __name__ == "__main__":
    pts = [(0, 0), (5, 4), (3, 1)]
    print(k_nearest(pts, (1, 2), 2))  # [(0, 0), (3, 1)]
