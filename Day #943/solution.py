# Day 943: Count tilings of a 2xN board with 2x1 dominoes and L-trominoes.
# DP recurrence f(n) = 2*f(n-1) + f(n-3), f(0)=1,f(1)=1,f(2)=2. Time O(n), Space O(1).


def count_tilings(n):
    if n == 0:
        return 1
    if n == 1:
        return 1
    if n == 2:
        return 2
    a, b, c = 1, 1, 2  # f(0), f(1), f(2)
    for _ in range(3, n + 1):
        a, b, c = b, c, 2 * c + a
    return c


if __name__ == "__main__":
    print(count_tilings(4))  # 11
