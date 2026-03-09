# Day 1186: Count intersecting segment pairs: sort segments by p, then count inversions in q.
# Two segments cross iff their p-order and q-order disagree. O(n log n) time, O(n) space.

def merge_count(a):
    if len(a) <= 1:
        return a, 0
    m = len(a) // 2
    left, cl = merge_count(a[:m])
    right, cr = merge_count(a[m:])
    merged, c = [], cl + cr
    i = j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            merged.append(left[i]); i += 1
        else:
            merged.append(right[j]); j += 1
            c += len(left) - i
    merged += left[i:]
    merged += right[j:]
    return merged, c


def count_intersections(p, q):
    order = sorted(range(len(p)), key=lambda k: p[k])
    qq = [q[k] for k in order]
    _, c = merge_count(qq)
    return c


if __name__ == "__main__":
    p, q = [1, 2, 3], [3, 1, 2]
    print(count_intersections(p, q))  # 2
