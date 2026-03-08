# Day 1178: Collatz sequences. Verify each reaches 1 and find the longest start <= 1e6.
# Iterative memoization caching every value along each chain. Time ~O(LIMIT), Space O(LIMIT).

LIMIT = 1000000


def main():
    memo = [0] * (LIMIT + 1)
    memo[1] = 1

    # Sample sequence for n = 6.
    seq, n = [], 6
    while True:
        seq.append(str(n))
        if n == 1:
            break
        n = n // 2 if n % 2 == 0 else 3 * n + 1
    print(" -> ".join(seq))

    for start in range(2, LIMIT + 1):
        if memo[start]:
            continue
        path, n = [], start
        while n > LIMIT or memo[n] == 0:
            path.append(n)
            n = n // 2 if n % 2 == 0 else 3 * n + 1
        base = memo[n]
        for v in reversed(path):
            base += 1
            if v <= LIMIT:
                memo[v] = base

    best_n = max(range(1, LIMIT + 1), key=lambda i: memo[i])
    print(f"All sequences up to {LIMIT} reach 1: true")
    print(f"Longest sequence for n <= {LIMIT}: n = {best_n} (length {memo[best_n]})")


if __name__ == "__main__":
    main()
