# Day 953: largest sum of non-adjacent numbers (may pick none -> >= 0).
# incl/excl DP. Time O(n), Space O(1).

def max_non_adjacent(a):
    incl, excl = 0, 0
    for x in a:
        incl, excl = excl + x, max(incl, excl)
    return max(incl, excl)


if __name__ == "__main__":
    print(max_non_adjacent([2, 4, 6, 2, 5]))  # 13
    print(max_non_adjacent([5, 1, 1, 5]))     # 10
