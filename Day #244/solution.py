# Day 244: Sieve of Eratosthenes: primes < N in O(N log log N) time, O(N) space.
# Plus an indefinite prime generator (Python generator) using incremental trial division by known primes.


def sieve(n):
    comp = [False] * max(n, 0)
    primes = []
    for i in range(2, n):
        if not comp[i]:
            primes.append(i)
            for j in range(i * i, n, i):
                comp[j] = True
    return primes


def prime_gen():
    """Indefinite generator yielding primes forever."""
    primes = []
    cand = 1
    while True:
        cand += 1
        is_prime = True
        for p in primes:
            if p * p > cand:
                break
            if cand % p == 0:
                is_prime = False
                break
        if is_prime:
            primes.append(cand)
            yield cand


def main():
    print(" ".join(map(str, sieve(100))))

    g = prime_gen()
    print(" ".join(str(next(g)) for _ in range(10)))


if __name__ == "__main__":
    main()
