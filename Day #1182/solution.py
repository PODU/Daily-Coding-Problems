# Day 1182: Split N into k contiguous partitions minimizing the maximum sum.
# Binary search the answer in [max element, total]; greedy feasibility check.
# Time O(N log(sum)), Space O(1).


def split_array(a, k):
    def feasible(cap):
        cur, parts = 0, 1
        for x in a:
            if cur + x > cap:
                parts += 1
                cur = x
                if parts > k:
                    return False
            else:
                cur += x
        return True

    lo, hi = max(a), sum(a)
    while lo < hi:
        mid = (lo + hi) // 2
        if feasible(mid):
            hi = mid
        else:
            lo = mid + 1
    return lo


if __name__ == "__main__":
    print(split_array([5, 1, 2, 7, 3, 4], 3))  # 8
