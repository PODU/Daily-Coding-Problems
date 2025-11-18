# Day 618: Pancake sort using only reverse(lst,i,j): flip current max to front, then to its place.
# Time: O(n^2) comparisons, O(n) flips, Space: O(1).


def reverse(lst, i, j):
    while i < j:
        lst[i], lst[j] = lst[j], lst[i]
        i += 1
        j -= 1


def pancake_sort(lst):
    for size in range(len(lst), 1, -1):
        max_idx = 0
        for k in range(1, size):
            if lst[k] > lst[max_idx]:
                max_idx = k
        if max_idx != size - 1:
            if max_idx != 0:
                reverse(lst, 0, max_idx)   # bring max to front
            reverse(lst, 0, size - 1)      # flip to final position
    return lst


if __name__ == "__main__":
    data = [3, 1, 4, 1, 5, 9, 2, 6]
    print(pancake_sort(data))
