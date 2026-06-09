# Day 1632: Min subset-sum difference: 0/1 subset-sum DP over reachable sums, pick best<=total/2; backtrack subset. O(n*total) time/space.


def main():
    a = [5, 10, 15, 20, 25]
    n = len(a)
    total = sum(a)

    # dp[i][s] = sum s reachable using first i items
    dp = [[False] * (total + 1) for _ in range(n + 1)]
    dp[0][0] = True
    for i in range(1, n + 1):
        for s in range(total + 1):
            dp[i][s] = dp[i - 1][s]
            if s >= a[i - 1] and dp[i - 1][s - a[i - 1]]:
                dp[i][s] = True

    best = 0
    for s in range(total // 2, -1, -1):
        if dp[n][s]:
            best = s
            break

    # Backtrack from last item to first to recover subset A
    A = []
    used = [False] * n
    s = best
    for i in range(n, 0, -1):
        if s >= a[i - 1] and dp[i - 1][s - a[i - 1]]:
            A.append(a[i - 1])
            used[i - 1] = True
            s -= a[i - 1]
    A.sort()

    B = [a[i] for i in range(n) if not used[i]]

    print("Minimum difference:", total - 2 * best)
    print("Subset A:", "[" + ", ".join(map(str, A)) + "]")
    print("Subset B:", "[" + ", ".join(map(str, B)) + "]")


if __name__ == "__main__":
    main()
