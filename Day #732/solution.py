# Day 732: Minimum boats (each holds <=2 people, weight limit k).
# Approach: Sort; two pointers pair lightest with heaviest when they fit.
# Time: O(n log n), Space: O(1).

def num_boats(weights, k):
    weights = sorted(weights)
    i, j, boats = 0, len(weights) - 1, 0
    while i <= j:
        if weights[i] + weights[j] <= k:
            i += 1
        j -= 1
        boats += 1
    return boats


if __name__ == "__main__":
    print(num_boats([100, 200, 150, 80], 200))  # 3
