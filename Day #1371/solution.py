# Day 1371: Assign mice to holes minimizing max distance: sort both, pair in order.
# Time O(n log n), Space O(1) extra.


def min_last_mouse(mice, holes):
    mice = sorted(mice)
    holes = sorted(holes)
    return max(abs(m - h) for m, h in zip(mice, holes))


if __name__ == "__main__":
    print(min_last_mouse([1, 4, 9, 15], [10, -5, 0, 16]))
