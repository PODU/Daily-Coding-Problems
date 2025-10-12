# Day 418: Two-pass greedy. Each gets >= 1; more than any lower neighbor. Like candy distribution.
# Time O(n), Space O(n).


def bonuses(lines):
    n = len(lines)
    res = [1] * n
    for i in range(1, n):
        if lines[i] > lines[i - 1]:
            res[i] = res[i - 1] + 1
    for i in range(n - 2, -1, -1):
        if lines[i] > lines[i + 1]:
            res[i] = max(res[i], res[i + 1] + 1)
    return res


if __name__ == "__main__":
    print(bonuses([10, 40, 200, 1000, 60, 30]))
