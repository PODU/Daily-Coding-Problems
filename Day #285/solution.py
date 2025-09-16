# Day 285: Count buildings (east->west) with a sunset (west) view.
# Single backward pass tracking running max. Time O(N), Space O(1).


def sunset_views(heights):
    count = 0
    max_so_far = float("-inf")
    for h in reversed(heights):  # from west end inward
        if h > max_so_far:
            count += 1
            max_so_far = h
    return count


if __name__ == "__main__":
    print(sunset_views([3, 7, 8, 3, 6, 1]))  # 3
