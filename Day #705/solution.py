# Day 705: Trapping rain water.
# Approach: two pointers tracking running left/right maxima; the smaller side is
# bounded so we can resolve it. Time O(N), Space O(1).


def trap(h):
    l, r = 0, len(h) - 1
    lmax = rmax = water = 0
    while l < r:
        if h[l] < h[r]:
            lmax = max(lmax, h[l])
            water += lmax - h[l]
            l += 1
        else:
            rmax = max(rmax, h[r])
            water += rmax - h[r]
            r -= 1
    return water


if __name__ == "__main__":
    print(trap([2, 1, 2]))           # 1
    print(trap([3, 0, 1, 3, 0, 5]))  # 8
