# Day 1758: Dutch national flag — segregate R, G, B in-place.
# Three pointers (low/mid/high), one pass. O(n) time, O(1) space.


def sort_rgb(a):
    low, mid, high = 0, 0, len(a) - 1
    while mid <= high:
        if a[mid] == 'R':
            a[low], a[mid] = a[mid], a[low]
            low += 1
            mid += 1
        elif a[mid] == 'G':
            mid += 1
        else:
            a[mid], a[high] = a[high], a[mid]
            high -= 1
    return a


def main():
    a = ['G', 'B', 'R', 'R', 'B', 'R', 'G']
    sort_rgb(a)
    print(a)


if __name__ == "__main__":
    main()
