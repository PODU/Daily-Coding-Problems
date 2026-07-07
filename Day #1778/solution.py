# Day 1778: Largest sum of non-adjacent numbers via include/exclude DP; empty selection (0) allowed.
# Time: O(N), Space: O(1).
def max_non_adjacent(a):
    incl, excl = 0, 0
    for n in a:
        ni = excl + n
        ne = max(incl, excl)
        incl, excl = ni, ne
    return max(incl, excl)


if __name__ == "__main__":
    print(max_non_adjacent([2, 4, 6, 2, 5]))
    print(max_non_adjacent([5, 1, 1, 5]))
