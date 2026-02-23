# Day 1119: Day 1119 - Split array into k contiguous partitions minimizing the max sum
# Approach: binary search the answer; greedily count partitions for a candidate
# max. Time: O(n log(sum)), Space: O(1).

def split_min_max(N, k):
    def partitions_needed(limit):
        count, cur = 1, 0
        for x in N:
            if cur + x > limit:
                count += 1
                cur = x
            else:
                cur += x
        return count

    lo, hi = max(N), sum(N)
    while lo < hi:
        mid = (lo + hi) // 2
        if partitions_needed(mid) <= k:
            hi = mid
        else:
            lo = mid + 1
    return lo


if __name__ == "__main__":
    N, k = [5, 1, 2, 7, 3, 4], 3
    print(split_min_max(N, k))  # 8
