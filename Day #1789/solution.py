# Day 1789: Min broadcast range: sort towers, binary-search nearest tower per listener, take max.
# Time O((N+M) log M), Space O(1) extra.
import bisect

def min_range(listeners, towers):
    towers = sorted(towers)
    ans = 0
    for L in listeners:
        i = bisect.bisect_left(towers, L)
        best = float("inf")
        if i < len(towers):
            best = min(best, towers[i] - L)
        if i > 0:
            best = min(best, L - towers[i - 1])
        ans = max(ans, best)
    return ans

if __name__ == "__main__":
    print(min_range([1, 5, 11, 20], [4, 8, 15]))
