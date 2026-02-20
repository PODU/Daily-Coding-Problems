# Day 1106: Count buildings (east->west) with a clear sunset view to the west.
# A building sees west if taller than all to its west; scan from west end, track max.
# Time: O(N) single pass. Space: O(1).
def sunset_views(h):
    count = 0
    max_so_far = float("-inf")
    for x in reversed(h):  # west -> east
        if x > max_so_far:
            count += 1
            max_so_far = x
    return count


if __name__ == "__main__":
    print(sunset_views([3, 7, 8, 3, 6, 1]))  # 3
