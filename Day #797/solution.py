# Day 797: Goldbach - two primes summing to even n, lexicographically smallest.
# Scan a from 2 upward; first prime a with prime (n-a) gives smallest pair.
# Time O(n * sqrt(n)), Space O(1).


def is_prime(x):
    if x < 2:
        return False
    d = 2
    while d * d <= x:
        if x % d == 0:
            return False
        d += 1
    return True


def goldbach(n):
    for a in range(2, n // 2 + 1):
        if is_prime(a) and is_prime(n - a):
            return a, n - a
    return -1, -1


if __name__ == "__main__":
    n = 4
    a, b = goldbach(n)
    print(f"{a} + {b} = {n}")  # 2 + 2 = 4
