# Day 147: Pancake sort: only primitive is reverse(lst,i,j). Each round reverse the window's max into place. O(n^2) time, O(1) space.


def reverse(lst, i, j):
    while i < j:
        lst[i], lst[j] = lst[j], lst[i]
        i += 1
        j -= 1


def pancake_sort(lst):
    n = len(lst)
    for size in range(n, 1, -1):
        max_idx = 0
        for k in range(1, size):
            if lst[k] > lst[max_idx]:
                max_idx = k
        if max_idx != size - 1:
            reverse(lst, max_idx, size - 1)
    return lst


if __name__ == "__main__":
    a = [3, 6, 1, 5, 2, 4]
    print(pancake_sort(a))  # [1, 2, 3, 4, 5, 6]
