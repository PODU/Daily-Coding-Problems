# Day 1189: 3-sum existence: sort, fix each i, two-pointer scan remaining pair. Time O(N^2), Space O(1).


def three_sum(arr, k):
    a = sorted(arr)
    n = len(a)
    for i in range(n - 2):
        lo, hi = i + 1, n - 1
        while lo < hi:
            s = a[i] + a[lo] + a[hi]
            if s == k:
                return True
            elif s < k:
                lo += 1
            else:
                hi -= 1
    return False


def main():
    arr = [20, 303, 3, 4, 25]
    print(str(three_sum(arr, 49)).lower())


if __name__ == '__main__':
    main()
