# Day 271: Search sorted list without *, /, or bit-shift -> Fibonacci search.
# Uses only addition/subtraction/comparison. Time O(log N), Space O(1).


def fib_search(arr, x):
    n = len(arr)
    fib_mm2, fib_mm1 = 0, 1
    fib_m = fib_mm2 + fib_mm1
    while fib_m < n:
        fib_mm2, fib_mm1 = fib_mm1, fib_m
        fib_m = fib_mm2 + fib_mm1
    offset = -1
    while fib_m > 1:
        i = min(offset + fib_mm2, n - 1)
        if arr[i] < x:
            fib_m, fib_mm1 = fib_mm1, fib_mm2
            fib_mm2 = fib_m - fib_mm1
            offset = i
        elif arr[i] > x:
            fib_m = fib_mm2
            fib_mm1 = fib_mm1 - fib_mm2
            fib_mm2 = fib_m - fib_mm1
        else:
            return i
    if fib_mm1 and offset + 1 < n and arr[offset + 1] == x:
        return offset + 1
    return -1


if __name__ == "__main__":
    arr = [1, 3, 4, 7, 9, 11, 15]
    print("7 -> index", fib_search(arr, 7))  # 3
    print("8 -> index", fib_search(arr, 8))  # -1
