# Day 924: Smallest window to sort: scan left->right tracking max for right bound,
# right->left tracking min for left bound. Time O(n), Space O(1).

def find_unsorted_window(arr):
    n = len(arr)
    right = -1
    running_max = arr[0]
    for i in range(1, n):
        if arr[i] < running_max:
            right = i
        else:
            running_max = arr[i]
    left = -1
    running_min = arr[n - 1]
    for i in range(n - 2, -1, -1):
        if arr[i] > running_min:
            left = i
        else:
            running_min = arr[i]
    return (left, right)


def main():
    arr = [3, 7, 5, 6, 9]
    print(find_unsorted_window(arr))


if __name__ == "__main__":
    main()
