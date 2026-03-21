# Day 1247: Squares of a sorted array via two-pointer merge from both ends. O(n) time, O(n) space.

def sorted_squares(a):
    n = len(a)
    res = [0] * n
    l, r = 0, n - 1
    for i in range(n - 1, -1, -1):
        ls, rs = a[l] * a[l], a[r] * a[r]
        if ls > rs:
            res[i] = ls
            l += 1
        else:
            res[i] = rs
            r -= 1
    return res


def main():
    input_arr = [-9, -2, 0, 2, 3]
    res = sorted_squares(input_arr)
    print("[" + ", ".join(str(x) for x in res) + "]")


if __name__ == "__main__":
    main()
