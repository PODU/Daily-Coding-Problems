# Day 766: Minimum flips so all 'x' come before all 'y'.
# One-pass DP: flips = min(flips+1, count_y). O(n) time, O(1) space.


def min_flips(s):
    flips = count_y = 0
    for c in s:
        if c == 'y':
            count_y += 1
        else:
            flips = min(flips + 1, count_y)
    return flips


if __name__ == "__main__":
    print(min_flips("xyxxxyxyy"))  # 2
