# Day 1257: Min broadcast range: sort towers, binary-search nearest per listener, answer = max of mins.
# Time O((N+M) log M), Space O(1).
import bisect

def min_range(listeners, towers):
    towers = sorted(towers)
    ans = 0
    for l in listeners:
        i = bisect.bisect_left(towers, l)
        best = float("inf")
        if i < len(towers):
            best = min(best, towers[i] - l)
        if i > 0:
            best = min(best, l - towers[i - 1])
        ans = max(ans, best)
    return ans

if __name__ == "__main__":
    print(min_range([1, 5, 11, 20], [4, 8, 15]))
