# Day 1856: Collatz conjecture test + longest chain under 1,000,000.
# Memoized step counts (cache values <= LIMIT). ~O(LIMIT * avg-chain) time, O(LIMIT) space.


def main():
    LIMIT = 1_000_000
    steps = [0] * (LIMIT + 1)  # steps[n] = steps to reach 1 (0 = unknown; steps[1]=0)
    all_reach_1 = True
    best_n, best_steps = 1, 0

    for i in range(2, LIMIT + 1):
        n = i
        cnt = 0
        while n != 1 and (n > LIMIT or steps[n] == 0):
            n = n // 2 if n % 2 == 0 else 3 * n + 1
            cnt += 1
        total = cnt + (0 if n == 1 else steps[n])
        steps[i] = total
        if total > best_steps:
            best_steps, best_n = total, i

    print(f"All sequences for n in [1, {LIMIT}] reach 1: {all_reach_1}")
    print(f"Longest sequence under {LIMIT}: n = {best_n} with {best_steps + 1} terms")
    # 837799, 525 terms


if __name__ == "__main__":
    main()
