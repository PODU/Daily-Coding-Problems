# Day 1807: Split array into k contiguous partitions minimizing the max partition sum.
# Binary search on the answer + greedy feasibility. Time: O(n log(sum)). Space: O(1).


def feasible(a, k, cap):
    cur, parts = 0, 1
    for x in a:
        if x > cap:
            return False
        if cur + x > cap:
            parts += 1
            cur = x
        else:
            cur += x
    return parts <= k


def split_array(a, k):
    lo, hi = max(a), sum(a)
    while lo < hi:
        mid = (lo + hi) // 2
        if feasible(a, k, mid):
            hi = mid
        else:
            lo = mid + 1
    return lo


if __name__ == "__main__":
    print(split_array([5, 1, 2, 7, 3, 4], 3))  # 8
