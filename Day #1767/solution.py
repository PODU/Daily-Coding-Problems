# Day 1767: Trapping Rain Water: two-pointer sweep tracking left/right running maxima.
# Time: O(N), Space: O(1).


def trap(h):
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
    print(trap([2, 1, 2]))
    print(trap([3, 0, 1, 3, 0, 5]))
