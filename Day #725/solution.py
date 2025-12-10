# Day 725: Assign mice to holes minimizing the maximum distance any mouse moves.
# Approach: Sort both arrays, pair in order, answer = max |mice[i]-holes[i]|.
# Time: O(n log n), Space: O(1).

def min_last_mouse(mice, holes):
    mice, holes = sorted(mice), sorted(holes)
    return max(abs(m - h) for m, h in zip(mice, holes))


if __name__ == "__main__":
    print(min_last_mouse([1, 4, 9, 15], [10, -5, 0, 16]))  # 6
