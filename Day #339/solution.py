# Day 339: Three entries summing to k: sort + fix one + two-pointer.
# O(n^2) time, O(1) extra space.


def three_sum(a, k):
    a = sorted(a)
    n = len(a)
    for i in range(n - 2):
        lo, hi = i + 1, n - 1
        target = k - a[i]
        while lo < hi:
            s = a[lo] + a[hi]
            if s == target:
                return True
            if s < target:
                lo += 1
            else:
                hi -= 1
    return False


def main():
    a = [20, 303, 3, 4, 25]
    print("true" if three_sum(a, 49) else "false")


if __name__ == "__main__":
    main()
