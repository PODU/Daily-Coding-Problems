# Day 1175: Maximum subarray sum in a circular array (empty allowed -> 0).
# Answer = max(0, kadaneMax, total - kadaneMin); total-min covers the wrap case.
# Time O(N), Space O(1).


def max_circular_subarray(a):
    total = cur_max = best_max = cur_min = best_min = 0
    for x in a:
        total += x
        cur_max = max(x, cur_max + x)
        best_max = max(best_max, cur_max)
        cur_min = min(x, cur_min + x)
        best_min = min(best_min, cur_min)
    return max(0, best_max, total - best_min)


if __name__ == "__main__":
    print(max_circular_subarray([8, -1, 3, 4]))  # 15
    print(max_circular_subarray([-4, 5, 1, 0]))  # 6
