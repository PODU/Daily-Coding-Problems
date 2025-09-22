# Day 314: Min broadcast range = max over listeners of distance to nearest tower.
# Sort towers, binary search each listener. O((N+M) log M).
import bisect


def min_range(listeners, towers):
    towers = sorted(towers)
    ans = 0
    for L in listeners:
        idx = bisect.bisect_left(towers, L)
        best = float("inf")
        if idx < len(towers):
            best = min(best, towers[idx] - L)
        if idx > 0:
            best = min(best, L - towers[idx - 1])
        ans = max(ans, best)
    return ans


if __name__ == "__main__":
    print(min_range([1, 5, 11, 20], [4, 8, 15]))  # 5
