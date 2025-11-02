# Day 535: Egg drop (min worst-case trials): find smallest m such that with N eggs we can
# cover k floors. f(m, N) = sum_{i=1..N} C(m,i); smallest m with f(m,N) >= k.
# Time: O(m * N) where m is the answer; Space: O(1).


def egg_drop(n, k):
    m = 0
    covered = 0
    while covered < k:
        m += 1
        total = 0
        term = 1  # term = C(m, i)
        for i in range(1, n + 1):
            term = term * (m - i + 1) // i  # C(m, i)
            total += term
            if total >= k:
                break
        covered = total
    return m


if __name__ == "__main__":
    print(egg_drop(1, 5))  # expected 5
