# Day 1006: Count intersecting segment pairs: sort by p, count inversions in q via merge sort.
# Time O(n log n), Space O(n).

def merge_count(a):
    if len(a) <= 1:
        return a, 0
    m = len(a) // 2
    left, cl = merge_count(a[:m])
    right, cr = merge_count(a[m:])
    merged = []
    i = j = 0
    cnt = cl + cr
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            merged.append(left[i]); i += 1
        else:
            merged.append(right[j]); j += 1
            cnt += len(left) - i
    merged.extend(left[i:])
    merged.extend(right[j:])
    return merged, cnt


def count_intersections(p, q):
    order = sorted(range(len(p)), key=lambda i: p[i])
    qs = [q[i] for i in order]
    _, cnt = merge_count(qs)
    return cnt


if __name__ == "__main__":
    p = [1, 2, 3, 4]
    q = [4, 3, 2, 1]
    print(count_intersections(p, q))  # 6
