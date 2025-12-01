# Day 679: Count inversions via modified merge sort. Time O(N log N), Space O(N).

def count_inversions(arr):
    def sort_count(a):
        n = len(a)
        if n <= 1:
            return a, 0
        mid = n // 2
        left, il = sort_count(a[:mid])
        right, ir = sort_count(a[mid:])
        merged = []
        i = j = inv = 0
        while i < len(left) and j < len(right):
            if left[i] <= right[j]:
                merged.append(left[i]); i += 1
            else:
                merged.append(right[j]); j += 1
                inv += len(left) - i
        merged.extend(left[i:])
        merged.extend(right[j:])
        return merged, il + ir + inv
    return sort_count(arr)[1]


if __name__ == "__main__":
    print(count_inversions([2, 4, 1, 3, 5]))
    print(count_inversions([5, 4, 3, 2, 1]))
