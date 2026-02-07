# Day 1035: Smallest bonuses so each employee beats any lower-output neighbor.
# Two-pass greedy: left-to-right then right-to-left taking max. Time O(n), Space O(n).


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
    a = [10, 40, 200, 1000, 60, 30]
    print(bonuses(a))
