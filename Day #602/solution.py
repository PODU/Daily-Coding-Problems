# Day 602: Approach: sort segments by p, then count inversions in the q-order via merge sort.
# Two segments cross iff their p-order and q-order disagree => an inversion. Time O(n log n), Space O(n).

def merge_count(a):
    if len(a) <= 1:
        return a, 0
    m = len(a) // 2
    left, il = merge_count(a[:m])
    right, ir = merge_count(a[m:])
    merged, inv = [], il + ir
    i = j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            merged.append(left[i]); i += 1
        else:
            merged.append(right[j]); j += 1
            inv += len(left) - i
    merged.extend(left[i:])
    merged.extend(right[j:])
    return merged, inv


def count_intersections(p, q):
    order = sorted(range(len(p)), key=lambda k: p[k])
    qs = [q[k] for k in order]
    _, inv = merge_count(qs)
    return inv


if __name__ == "__main__":
    p = [1, 2, 3, 4]
    q = [4, 3, 2, 1]
    print(count_intersections(p, q))
