# Day 1616: Goldbach pair: sieve up to n, scan a from 2; first a where a and n-a prime.
# Smallest a => lexicographically smallest [a,b]. Time O(n log log n), Space O(n).

def goldbach(n):
    is_prime = [True] * (n + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(n ** 0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, n + 1, i):
                is_prime[j] = False
    a = 2
    while a <= n - a:
        if is_prime[a] and is_prime[n - a]:
            return a, n - a
        a += 1
    return -1, -1


if __name__ == "__main__":
    n = 4
    a, b = goldbach(n)
    print(f"{a} + {b} = {n}")
