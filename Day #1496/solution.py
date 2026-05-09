# Day 1496: Min cost to paint N houses with K colors, no two adjacent same color.
# Approach: DP tracking previous row's min & 2nd-min (+min index). Time O(N*K), Space O(1).


def min_cost(costs):
    if not costs:
        return 0
    prev_min1 = prev_min2 = 0
    prev_idx = -1
    for row in costs:
        cur_min1 = cur_min2 = float("inf")
        cur_idx = -1
        for k, base in enumerate(row):
            add = prev_min2 if k == prev_idx else prev_min1
            c = base + add
            if c < cur_min1:
                cur_min2, cur_min1, cur_idx = cur_min1, c, k
            elif c < cur_min2:
                cur_min2 = c
        prev_min1, prev_min2, prev_idx = cur_min1, cur_min2, cur_idx
    return prev_min1


if __name__ == "__main__":
    costs = [[1, 5, 3], [2, 9, 4]]
    print(min_cost(costs))  # expected 5
