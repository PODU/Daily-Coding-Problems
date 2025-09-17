# Day 291: Boats: <=2 people, weight limit k, minimize boats. Sort + two pointers.
# Pair lightest with heaviest if they fit, else heaviest alone. Time O(n log n), Space O(1).
def num_boats(weights, k):
    w = sorted(weights)
    l, h, boats = 0, len(w) - 1, 0
    while l <= h:
        if w[l] + w[h] <= k:
            l += 1
        h -= 1
        boats += 1
    return boats


if __name__ == "__main__":
    print(num_boats([100, 200, 150, 80], 200))
