# Day 118: Two-pointer merge from both ends into result back-to-front. O(n) time, O(n) space.
def sorted_squares(a):
    n = len(a)
    res = [0] * n
    lo, hi = 0, n - 1
    for k in range(n - 1, -1, -1):
        sl, sh = a[lo] * a[lo], a[hi] * a[hi]
        if sl > sh:
            res[k] = sl
            lo += 1
        else:
            res[k] = sh
            hi -= 1
    return res


if __name__ == "__main__":
    print(sorted_squares([-9, -2, 0, 2, 3]))  # [0, 4, 4, 9, 81]
