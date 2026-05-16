# Day 1528: Boat rescue: min boats, <=2 people each, weight limit k.
# Greedy two-pointer: sort, pair lightest with heaviest if sum<=k. O(n log n) time, O(1) extra.

def num_rescue_boats(w, k):
    w = sorted(w)
    lo, hi, boats = 0, len(w) - 1, 0
    while lo <= hi:
        if w[lo] + w[hi] <= k:
            lo += 1
        hi -= 1
        boats += 1
    return boats


if __name__ == "__main__":
    print(num_rescue_boats([100, 200, 150, 80], 200))  # expected 3
