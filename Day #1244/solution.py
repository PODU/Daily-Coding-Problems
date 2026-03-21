# Day 1244: Boats: sort weights, greedily pair lightest with heaviest (two-pointer).
# Time O(n log n), Space O(1).


def num_boats(weights, k):
    weights = sorted(weights)
    i, j = 0, len(weights) - 1
    boats = 0
    while i <= j:
        if weights[i] + weights[j] <= k:
            i += 1
        j -= 1
        boats += 1
    return boats


if __name__ == "__main__":
    print(num_boats([100, 200, 150, 80], 200))
