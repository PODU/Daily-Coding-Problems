# Day 707: Min broadcast range. Sort towers; for each listener binary-search the
# nearest tower, answer is max of those min distances. Time O((N+M)logM).
import bisect


def min_range(listeners, towers):
    towers = sorted(towers)
    ans = 0
    for x in listeners:
        i = bisect.bisect_left(towers, x)
        best = float("inf")
        if i < len(towers):
            best = min(best, towers[i] - x)
        if i > 0:
            best = min(best, x - towers[i - 1])
        ans = max(ans, best)
    return ans


if __name__ == "__main__":
    print(min_range([1, 5, 11, 20], [4, 8, 15]))
