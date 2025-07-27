# Day 35: Dutch National Flag 3-way partition (R<G<B). In-place, O(n) time, O(1) space, swaps only.
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


if __name__ == "__main__":
    arr = ['G', 'B', 'R', 'R', 'B', 'R', 'G']
    sort_rgb(arr)
    print("[" + ", ".join("'" + c + "'" for c in arr) + "]")
