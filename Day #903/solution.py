# Day 903: Sort, then fix arr[i] and two-pointer scan for the remaining pair summing to k.
# Time O(n^2), Space O(1) extra.
def three_sum(arr, k):
    arr = sorted(arr)
    n = len(arr)
    for i in range(n - 2):
        lo, hi = i + 1, n - 1
        while lo < hi:
            s = arr[i] + arr[lo] + arr[hi]
            if s == k:
                return True
            if s < k:
                lo += 1
            else:
                hi -= 1
    return False


if __name__ == "__main__":
    arr = [20, 303, 3, 4, 25]
    found = three_sum(arr, 49)
    print("true" if found else "false")
