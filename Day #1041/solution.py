# Day 1041: Trapping rain water via two pointers tracking leftMax/rightMax. Time O(N), Space O(1).

def trap(h):
    l, r = 0, len(h) - 1
    lm = rm = water = 0
    while l < r:
        if h[l] < h[r]:
            lm = max(lm, h[l])
            water += lm - h[l]
            l += 1
        else:
            rm = max(rm, h[r])
            water += rm - h[r]
            r -= 1
    return water


if __name__ == "__main__":
    print("[2, 1, 2] ->", trap([2, 1, 2]))
    print("[3, 0, 1, 3, 0, 5] ->", trap([3, 0, 1, 3, 0, 5]))
