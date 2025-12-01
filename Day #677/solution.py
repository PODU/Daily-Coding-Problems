# Day 677: Sieve of Eratosthenes for primes < N (O(N log log N)); plus an incremental
# generator using a dict of next composite multiples. Space: O(N) sieve.

def sieve(n):
    is_comp = [False] * max(0, n)
    primes = []
    for i in range(2, n):
        if not is_comp[i]:
            primes.append(i)
            for j in range(i * i, n, i):
                is_comp[j] = True
    return primes


def prime_gen():
    # Incremental sieve: dict maps next composite -> its prime factor.
    composites = {}
    current = 1
    while True:
        current += 1
        if current not in composites:
            composites[current * current] = current  # prime found
            yield current
        else:
            prime = composites.pop(current)
            x = current + prime
            while x in composites:
                x += prime
            composites[x] = prime


def main():
    print(sieve(100))
    gen = prime_gen()
    print([next(gen) for _ in range(10)])


if __name__ == "__main__":
    main()
