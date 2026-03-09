# Day 1188: Count inversions via merge sort: count cross-pairs while merging. Time O(N log N), Space O(N).


def count_inversions(arr):
    def sort_count(a):
        n = len(a)
        if n <= 1:
            return a, 0
        mid = n // 2
        left, l_inv = sort_count(a[:mid])
        right, r_inv = sort_count(a[mid:])
        merged = []
        i = j = 0
        inv = l_inv + r_inv
        while i < len(left) and j < len(right):
            if left[i] <= right[j]:
                merged.append(left[i])
                i += 1
            else:
                merged.append(right[j])
                j += 1
                inv += len(left) - i
        merged.extend(left[i:])
        merged.extend(right[j:])
        return merged, inv

    _, total = sort_count(arr)
    return total


def main():
    arr = [2, 4, 1, 3, 5]
    print(count_inversions(arr))


if __name__ == '__main__':
    main()
