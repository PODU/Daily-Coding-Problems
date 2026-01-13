# Day 893: Paint houses: DP tracking two smallest costs of previous row -> O(N*K) time, O(1) extra space.
# For each house we only need the min and second-min of the previous row to avoid same color.

def min_cost(costs):
    if not costs:
        return 0
    prev_min, prev_second, prev_idx = 0, 0, -1
    for row in costs:
        cur_min, cur_second, cur_idx = float('inf'), float('inf'), -1
        for c, base in enumerate(row):
            cost = base + (prev_second if c == prev_idx else prev_min)
            if cost < cur_min:
                cur_second = cur_min
                cur_min = cost
                cur_idx = c
            elif cost < cur_second:
                cur_second = cost
        prev_min, prev_second, prev_idx = cur_min, cur_second, cur_idx
    return prev_min


def main():
    costs = [[1, 5, 3], [2, 9, 4]]
    print(min_cost(costs))


if __name__ == "__main__":
    main()
