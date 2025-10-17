# Day 448: Dutch National Flag sort of R/G/B. O(n) time, O(1) space, in-place
# with three pointers (low=R boundary, high=B boundary, mid=scanner).


def sort_rgb(a):
    low, mid, high = 0, 0, len(a) - 1
    while mid <= high:
        if a[mid] == 'R':
            a[low], a[mid] = a[mid], a[low]
            low += 1
            mid += 1
        elif a[mid] == 'G':
            mid += 1
        else:  # 'B'
            a[mid], a[high] = a[high], a[mid]
            high -= 1
    return a


if __name__ == "__main__":
    print(sort_rgb(['G', 'B', 'R', 'R', 'B', 'R', 'G']))
    # ['R', 'R', 'R', 'G', 'G', 'B', 'B']
