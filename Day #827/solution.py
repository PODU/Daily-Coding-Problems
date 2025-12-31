# Day 827: Min broadcast range.
# Sort towers; for each listener binary-search nearest tower, take min distance;
# answer = max over listeners. Time O((N+M) log M), Space O(1).
import bisect


def min_broadcast_range(listeners, towers):
    towers = sorted(towers)
    ans = 0
    for l in listeners:
        i = bisect.bisect_left(towers, l)
        best = float('inf')
        if i < len(towers):
            best = min(best, towers[i] - l)
        if i > 0:
            best = min(best, l - towers[i - 1])
        ans = max(ans, best)
    return ans


if __name__ == "__main__":
    print(min_broadcast_range([1, 5, 11, 20], [4, 8, 15]))
