# Day 206: Apply permutation P to array (result[P[i]] = a[i]).
# In-place via cycle following on the permutation. Time: O(n), Space: O(1).


def apply_permutation(a, p):
    p = list(p)
    for i in range(len(a)):
        while p[i] != i:
            j = p[i]
            a[i], a[j] = a[j], a[i]
            p[i], p[j] = p[j], j
    return a


if __name__ == "__main__":
    print(apply_permutation(["a", "b", "c"], [2, 1, 0]))  # ['c', 'b', 'a']
