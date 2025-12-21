# Day 773: Count inversions via modified merge sort. O(n log n) time, O(n) space.


def count_inversions(arr):
    def sort_count(a):
        if len(a) <= 1:
            return a, 0
        mid = len(a) // 2
        left, cl = sort_count(a[:mid])
        right, cr = sort_count(a[mid:])
        merged, c = [], cl + cr
        i = j = 0
        while i < len(left) and j < len(right):
            if left[i] <= right[j]:
                merged.append(left[i]); i += 1
            else:
                merged.append(right[j]); j += 1
                c += len(left) - i
        merged.extend(left[i:])
        merged.extend(right[j:])
        return merged, c

    return sort_count(arr)[1]


if __name__ == "__main__":
    print(count_inversions([2, 4, 1, 3, 5]))  # 3
    print(count_inversions([5, 4, 3, 2, 1]))  # 10
