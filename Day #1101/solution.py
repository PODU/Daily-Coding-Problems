# Day 1101: Assign mice to holes minimizing the maximum travel distance.
# Sort both, match in order; answer is max |mice[i]-holes[i]|.
# Time: O(N log N). Space: O(1).
def min_last_mouse(mice, holes):
    mice = sorted(mice)
    holes = sorted(holes)
    return max(abs(m - h) for m, h in zip(mice, holes))


if __name__ == "__main__":
    print(min_last_mouse([1, 4, 9, 15], [10, -5, 0, 16]))  # 6
