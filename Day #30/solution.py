# Day 30: Trapping rain water with two pointers.
# Time: O(n), Space: O(1).
from typing import List


def trap(h: List[int]) -> int:
    l, r = 0, len(h) - 1
    left_max = right_max = water = 0
    while l < r:
        if h[l] < h[r]:
            left_max = max(left_max, h[l])
            water += left_max - h[l]
            l += 1
        else:
            right_max = max(right_max, h[r])
            water += right_max - h[r]
            r -= 1
    return water


if __name__ == "__main__":
    heights = [3, 0, 1, 3, 0, 5]
    print(trap(heights))
