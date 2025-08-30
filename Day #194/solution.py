# Day 194: Segments p_i->q_i cross iff order of p and q disagree. Count = inversions of q
# after sorting pairs by p. Merge-sort inversion count. Time O(n log n), Space O(n).
from typing import List


def _merge_count(v: List[int]) -> int:
    if len(v) <= 1:
        return 0
    m = len(v) // 2
    left, right = v[:m], v[m:]
    cnt = _merge_count(left) + _merge_count(right)
    i = j = k = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            v[k] = left[i]; i += 1
        else:
            v[k] = right[j]; j += 1
            cnt += len(left) - i
        k += 1
    while i < len(left):
        v[k] = left[i]; i += 1; k += 1
    while j < len(right):
        v[k] = right[j]; j += 1; k += 1
    return cnt


def count_crossings(p: List[int], q: List[int]) -> int:
    order = sorted(range(len(p)), key=lambda i: p[i])
    qs = [q[i] for i in order]
    return _merge_count(qs)


if __name__ == "__main__":
    print(count_crossings([1, 2, 3, 4], [4, 3, 2, 1]))
