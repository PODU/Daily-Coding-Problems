# Day 1669: Two-pointer merge from both ends of sorted array; larger abs square goes to back. O(n) time, O(n) space.
def sorted_squares(a):
    n = len(a); r = [0]*n; l, h = 0, n-1
    for p in range(n-1, -1, -1):
        lo, hi = a[l]*a[l], a[h]*a[h]
        if lo > hi:
            r[p] = lo; l += 1
        else:
            r[p] = hi; h -= 1
    return r

print(sorted_squares([-9, -2, 0, 2, 3]))
