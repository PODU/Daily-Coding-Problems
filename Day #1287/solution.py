# Day 1287: Minimum bonuses so each employee out-earns lower-scoring neighbors.
# Two passes (left->right, right->left), take max. Time O(n), Space O(n).
def bonuses(a):
    n = len(a)
    b = [1] * n
    for i in range(1, n):
        if a[i] > a[i - 1]:
            b[i] = b[i - 1] + 1
    for i in range(n - 2, -1, -1):
        if a[i] > a[i + 1]:
            b[i] = max(b[i], b[i + 1] + 1)
    return b


if __name__ == "__main__":
    print(bonuses([10, 40, 200, 1000, 60, 30]))  # [1, 2, 3, 4, 2, 1]
