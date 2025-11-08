# Day 568: Square a sorted array and return sorted. Two pointers from both ends pick larger
# absolute value into the back of the result. O(N) time, O(N) space.

def sorted_squares(a):
    n = len(a)
    l, r = 0, n - 1
    res = [0] * n
    for k in range(n - 1, -1, -1):
        lv, rv = a[l] * a[l], a[r] * a[r]
        if lv > rv:
            res[k] = lv
            l += 1
        else:
            res[k] = rv
            r -= 1
    return res


if __name__ == "__main__":
    print(sorted_squares([-9, -2, 0, 2, 3]))  # [0, 4, 4, 9, 81]
