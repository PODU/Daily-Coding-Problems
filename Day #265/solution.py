# Day 265: Minimum bonuses. Two-pass scan (left-to-right then right-to-left),
# each worker gets max of the two passes. Time O(n), space O(n).


def bonuses(lines):
    n = len(lines)
    b = [1] * n
    for i in range(1, n):
        if lines[i] > lines[i - 1]:
            b[i] = b[i - 1] + 1
    for i in range(n - 2, -1, -1):
        if lines[i] > lines[i + 1]:
            b[i] = max(b[i], b[i + 1] + 1)
    return b


if __name__ == "__main__":
    lines = [10, 40, 200, 1000, 60, 30]
    print(bonuses(lines))
