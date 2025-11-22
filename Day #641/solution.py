# Day 641: Smallest positive integer not expressible as a subset sum.
# Approach: scan sorted array; if a[i] > reach+1 a gap exists, else reach += a[i].
# Time: O(N), Space: O(1).
def smallest_non_sum(a):
    reach = 0  # all of [1..reach] are representable
    for x in a:
        if x > reach + 1:
            break
        reach += x
    return reach + 1


if __name__ == "__main__":
    print(smallest_non_sum([1, 2, 3, 10]))  # 7
