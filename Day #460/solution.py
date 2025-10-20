# Day 460: Min flips so every 'x' precedes every 'y'.
# One-pass DP: dp = min(flip this x->y, flip all prior y->x). Time O(n), Space O(1).


def min_flips(s):
    dp = y = 0
    for c in s:
        if c == 'y':
            y += 1
        else:
            dp = min(dp + 1, y)
    return dp


if __name__ == "__main__":
    print(min_flips("xyxxxyxyy"))  # 2
