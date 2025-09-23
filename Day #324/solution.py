# Day 324: Assign mice to holes minimizing max distance: sort both, pair i-th, answer = max|mice[i]-holes[i]|.
# Time: O(N log N), Space: O(1) extra.

def min_max_distance(mice, holes):
    mice = sorted(mice)
    holes = sorted(holes)
    return max(abs(m - h) for m, h in zip(mice, holes))


if __name__ == "__main__":
    mice = [1, 4, 9, 15]
    holes = [10, -5, 0, 16]
    print(min_max_distance(mice, holes))
