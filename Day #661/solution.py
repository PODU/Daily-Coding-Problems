# Day 661: Search sorted array without mul/div/bit-shift.
# Fibonacci search uses only +/- to pick probe points. Time O(log N), Space O(1).


def fib_search(a, x):
    n = len(a)
    f2, f1 = 0, 1
    f = f1 + f2
    while f < n:
        f2, f1 = f1, f
        f = f1 + f2
    offset = -1
    while f > 1:
        i = min(offset + f2, n - 1)
        if a[i] < x:
            f, f1 = f1, f2
            f2 = f - f1
            offset = i
        elif a[i] > x:
            f = f2
            f1 = f1 - f2
            f2 = f - f1
        else:
            return i
    if f1 and offset + 1 < n and a[offset + 1] == x:
        return offset + 1
    return -1


if __name__ == "__main__":
    a = [-1, 0, 3, 5, 9, 12]
    print("Index of 5:", fib_search(a, 5))  # 3
    print("Index of 7:", fib_search(a, 7))  # -1 (absent)
