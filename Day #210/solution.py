# Day 210: Collatz conjecture - verify reach to 1 and find longest sequence for n <= 1e6.
# Memoized sequence lengths (cache for values <= LIMIT). Time: ~O(LIMIT * avg steps), Space: O(LIMIT).
import sys

LIMIT = 1000000


def main():
    cache = [0] * (LIMIT + 1)
    cache[1] = 1

    def collatz_len(start):
        path = []
        m = start
        while m > LIMIT or cache[m] == 0:
            path.append(m)
            m = m // 2 if m % 2 == 0 else 3 * m + 1
        base = cache[m]
        for v in reversed(path):
            base += 1
            if v <= LIMIT:
                cache[v] = base
        return base

    print("Collatz length of 27:", collatz_len(27))  # 112
    best_n, best_len = 1, 1
    for n in range(1, LIMIT + 1):
        l = collatz_len(n)
        if l > best_len:
            best_len, best_n = l, n
    print(f"Longest sequence for n <= 1000000: n={best_n} (length {best_len})")  # n=837799 (length 525)


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    main()
