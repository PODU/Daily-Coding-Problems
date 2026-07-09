# Day 1788: k nearest points: stable sort by squared Euclidean distance, take first k.
# Time O(N log N), Space O(N).

def k_nearest(points, central, k):
    cx, cy = central
    ordered = sorted(points, key=lambda p: (p[0]-cx)**2 + (p[1]-cy)**2)  # stable
    return ordered[:k]

if __name__ == "__main__":
    points = [(0, 0), (5, 4), (3, 1)]
    central = (1, 2)
    k = 2
    print(k_nearest(points, central, k))
