# Day 735: Find any peak element in O(log N).
# Approach: Binary search - move toward the larger neighbor; a peak must lie that way.
# Time: O(log n), Space: O(1).

def find_peak(a):
    lo, hi = 0, len(a) - 1
    while lo < hi:
        mid = (lo + hi) // 2
        if a[mid] < a[mid + 1]:
            lo = mid + 1
        else:
            hi = mid
    return lo


if __name__ == "__main__":
    a = [0, 2, 5, 3, 1]
    i = find_peak(a)
    print(f"Peak element: {a[i]} (index {i})")  # 5 (index 2)
