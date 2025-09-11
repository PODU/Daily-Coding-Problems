# Day 257: Smallest window that must be sorted to make the whole array sorted.
# Left->right track max to find right bound; right->left track min to find left bound.
# Time: O(n), Space: O(1).


def sort_window(a):
    n = len(a)
    begin = end = -1
    mx = float("-inf")
    for i in range(n):
        if a[i] < mx:
            end = i
        else:
            mx = a[i]
    mn = float("inf")
    for i in range(n - 1, -1, -1):
        if a[i] > mn:
            begin = i
        else:
            mn = a[i]
    return (begin, end)


def main():
    a = [3, 7, 5, 6, 9]
    print(sort_window(a))  # (1, 3)


if __name__ == "__main__":
    main()
