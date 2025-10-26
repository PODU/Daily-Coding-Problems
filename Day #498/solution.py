# Day 498: Smallest window to sort so the whole array is sorted.
# Two passes: left->right track max for right bound; right->left track min for left bound.
# Time: O(n), Space: O(1).


def window_to_sort(a):
    n = len(a)
    left = right = -1
    max_so_far = a[0]
    for i in range(1, n):
        if a[i] < max_so_far:
            right = i
        else:
            max_so_far = a[i]
    min_so_far = a[n - 1]
    for i in range(n - 2, -1, -1):
        if a[i] > min_so_far:
            left = i
        else:
            min_so_far = a[i]
    return left, right


def main():
    a = [3, 7, 5, 6, 9]
    left, right = window_to_sort(a)
    print("({}, {})".format(left, right))


if __name__ == "__main__":
    main()
