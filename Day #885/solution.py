# Day 885: Dutch national flag: three-pointer in-place partition R<G<B. Time O(n), Space O(1).

def sort_rgb(a):
    low, mid, high = 0, 0, len(a) - 1
    while mid <= high:
        if a[mid] == "R":
            a[low], a[mid] = a[mid], a[low]
            low += 1
            mid += 1
        elif a[mid] == "G":
            mid += 1
        else:  # 'B'
            a[mid], a[high] = a[high], a[mid]
            high -= 1
    return a


if __name__ == "__main__":
    a = ["G", "B", "R", "R", "B", "R", "G"]
    print(sort_rgb(a))
