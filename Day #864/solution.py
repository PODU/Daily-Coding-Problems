# Day 864: Minimum rescue boats (<=2 people, total weight <= k).
# Approach: sort, greedily pair lightest with heaviest using two pointers.
# Time: O(n log n), Space: O(1).


def num_rescue_boats(weights, k):
    weights.sort()
    i, j, boats = 0, len(weights) - 1, 0
    while i <= j:
        if weights[i] + weights[j] <= k:
            i += 1
        j -= 1
        boats += 1
    return boats


if __name__ == "__main__":
    print(num_rescue_boats([100, 200, 150, 80], 200))  # 3
