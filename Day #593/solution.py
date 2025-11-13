# Day 593: Count buildings with a view of the setting sun (west).
# Array is east->west (index 0 = east). A building sees the sunset iff it is
# taller than every building further west (higher index). Single right-to-left pass.
# Time: O(n), Space: O(1).


def count_sunset_views(heights):
    count = 0
    max_seen = float("-inf")
    for h in reversed(heights):  # from west end
        if h > max_seen:
            count += 1
            max_seen = h
    return count


if __name__ == "__main__":
    print(count_sunset_views([3, 7, 8, 3, 6, 1]))  # 3
