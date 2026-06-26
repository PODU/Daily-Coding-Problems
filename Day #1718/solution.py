# Day 1718: Count intersecting segment pairs (p_i on y=0 -> q_i on y=1).
# Two segments cross iff their (p, q) ordering is inverted: sort by p,
# count inversions in q via merge sort. Time: O(n log n), Space: O(n).


def _merge_count(a, l, r):
    if r - l <= 1:
        return 0
    mid = (l + r) // 2
    inv = _merge_count(a, l, mid) + _merge_count(a, mid, r)
    tmp = []
    i, j = l, mid
    while i < mid and j < r:
        if a[i] <= a[j]:
            tmp.append(a[i])
            i += 1
        else:
            inv += mid - i
            tmp.append(a[j])
            j += 1
    tmp.extend(a[i:mid])
    tmp.extend(a[j:r])
    a[l:r] = tmp
    return inv


def count_intersections(p, q):
    n = len(p)
    idx = sorted(range(n), key=lambda i: p[i])
    qs = [q[i] for i in idx]
    return _merge_count(qs, 0, n)


if __name__ == "__main__":
    p = [1, 2, 3, 4]
    q = [2, 1, 4, 3]
    print("Intersecting pairs:", count_intersections(p, q))
