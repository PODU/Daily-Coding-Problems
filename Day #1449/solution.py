# Day 1449: Trapping Rain Water. Two-pointer sweep tracking running left/right
# maxima. Time O(n), Space O(1).
from typing import List


def trap(h: List[int]) -> int:
    l, r = 0, len(h) - 1
    left_max = right_max = 0
    water = 0
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
    print(trap([2, 1, 2]))         # 1
    print(trap([3, 0, 1, 3, 0, 5]))  # 8
