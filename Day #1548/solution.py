# Day 1548: Candy problem: init bonuses to 1, left-to-right then right-to-left passes enforce strict ordering.
# Time O(n), Space O(n).

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
