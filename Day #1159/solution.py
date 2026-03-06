# Day 1159: Goldbach: iterate a from 2 upward, return first (a, n-a) both prime (lexicographically smallest).
# Time: O(n*sqrt(n)) worst, Space: O(1).
def is_prime(x):
    if x < 2:
        return False
    i = 2
    while i * i <= x:
        if x % i == 0:
            return False
        i += 1
    return True


def goldbach(n):
    a = 2
    while a <= n // 2:
        if is_prime(a) and is_prime(n - a):
            return a, n - a
        a += 1
    return -1, -1


if __name__ == "__main__":
    n = 4
    a, b = goldbach(n)
    print(f"{a} + {b} = {n}")
