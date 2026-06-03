# Day 1605: 2xN tiling with dominoes + L-trominoes (LeetCode 790). DP recurrence
# f(n)=2*f(n-1)+f(n-3). O(N) time, O(1) space. Mod 1e9+7 for large N.
MOD = 1_000_000_007


def num_tilings(n: int) -> int:
    if n == 0:
        return 1
    if n == 1:
        return 1
    if n == 2:
        return 2
    a, b, c = 1, 1, 2  # f(0), f(1), f(2)
    for _ in range(3, n + 1):
        a, b, c = b, c, (2 * c + a) % MOD
    return c


if __name__ == "__main__":
    print(f"N=4 -> {num_tilings(4)}")  # 11
    table = " ".join(str(num_tilings(n)) for n in range(1, 6))
    print(f"Table N=1..5: {table}")  # 1 2 5 11 24
