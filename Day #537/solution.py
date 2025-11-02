# Day 537: Collatz: iterate n->n/2 (even) or 3n+1 (odd) counting steps to 1; verify a range.
# Bonus longest under 1e6 via memoized step counts. Time ~O(LIMIT*avg-chain), space O(LIMIT).

def steps(n):
    c = 0
    while n != 1:
        n = n // 2 if n % 2 == 0 else 3 * n + 1
        c += 1
    return c

def main():
    all_reach = all(steps(n) >= 0 for n in range(1, 21))
    print("Collatz conjecture holds for 1..20: " + str(all_reach).lower())

    LIMIT = 1000000
    dp = [0] * (LIMIT + 1)
    best_n, best_len = 1, 0
    for i in range(2, LIMIT + 1):
        n, c = i, 0
        while n != 1 and (n > LIMIT or dp[n] == 0):
            n = n // 2 if n % 2 == 0 else 3 * n + 1
            c += 1
        c += 0 if n == 1 else dp[n]
        dp[i] = c
        if c > best_len:
            best_len, best_n = c, i
    print(f"Longest under 1000000: n={best_n} with {best_len} steps")

if __name__ == "__main__":
    main()
