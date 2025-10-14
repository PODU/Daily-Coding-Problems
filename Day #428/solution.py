# Day 428: Min cost to carve a strict pyramid (1,2,..,P,..,2,1); only lowering allowed.
# left[i]/right[i] cap each stone by distance from the ends; peak height H = max min(left,right).
# A strict tent of peak H has sum H*H, so cost = sum(stones) - H*H. Time O(n), Space O(n).
def solve(stones):
    n = len(stones)
    left = [0] * n
    right = [0] * n
    left[0] = min(stones[0], 1)
    for i in range(1, n):
        left[i] = min(stones[i], left[i - 1] + 1)
    right[n - 1] = min(stones[n - 1], 1)
    for i in range(n - 2, -1, -1):
        right[i] = min(stones[i], right[i + 1] + 1)
    best = [min(left[i], right[i]) for i in range(n)]
    H = max(best)
    p = best.index(H)
    cost = sum(stones) - H * H
    heights = [max(0, H - abs(i - p)) for i in range(n)]
    return cost, heights


if __name__ == "__main__":
    stones = [1, 1, 3, 3, 2, 1]
    cost, heights = solve(stones)
    print(f"{cost}  (resulting in {heights})")
