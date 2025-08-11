# Day 101: Goldbach. Sieve primes up to n, then take the smallest prime a with
# n-a also prime; that pair is lexicographically smallest. O(n log log n).
def goldbach(n):
    sieve = [True] * (n + 1)
    sieve[0] = sieve[1] = False
    for i in range(2, int(n ** 0.5) + 1):
        if sieve[i]:
            for j in range(i * i, n + 1, i):
                sieve[j] = False
    for a in range(2, n // 2 + 1):
        if sieve[a] and sieve[n - a]:
            return (a, n - a)
    return None


if __name__ == "__main__":
    a, b = goldbach(4)
    print(f"{a} + {b} = {a + b}")  # 2 + 2 = 4
