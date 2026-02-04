# Day 1017: Pancake sort using only reverse(lst,i,j): for each end, bring max of prefix to front then flip to its spot.
# O(n^2) comparisons, O(n) reversals, in place. Space O(1).
def reverse(lst, i, j):
    while i < j:
        lst[i], lst[j] = lst[j], lst[i]
        i += 1
        j -= 1


def pancake_sort(lst):
    n = len(lst)
    for end in range(n - 1, 0, -1):
        mi = 0
        for k in range(1, end + 1):
            if lst[k] > lst[mi]:
                mi = k
        if mi == end:
            continue
        if mi != 0:
            reverse(lst, 0, mi)   # bring max to front
        reverse(lst, 0, end)      # move max to its final position
    return lst


if __name__ == "__main__":
    arr = [3, 1, 4, 1, 5, 9, 2, 6]
    print(pancake_sort(arr))
