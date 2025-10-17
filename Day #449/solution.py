# Day 449: Goldbach pair. Scan a from 2 upward; the first a where a and n-a are
# both prime gives the lexicographically smallest pair. O(n*sqrt(n)) worst case.


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
    for a in range(2, n // 2 + 1):
        if is_prime(a) and is_prime(n - a):
            return a, n - a
    return None


if __name__ == "__main__":
    n = 4
    a, b = goldbach(n)
    print(f"{a} + {b} = {n}")  # 2 + 2 = 4
