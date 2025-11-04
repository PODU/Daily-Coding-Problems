# Day 552: Domino + Tromino tiling of 2xN: f(n)=2*f(n-1)+f(n-3), f(0)=1,f(1)=1,f(2)=2.
# O(n) time, O(1) space.


def tilings(n):
    if n == 0:
        return 1
    if n == 1:
        return 1
    if n == 2:
        return 2
    a, b, c = 1, 1, 2  # f(0),f(1),f(2)
    cur = c
    for _ in range(3, n + 1):
        cur = 2 * c + a
        a, b, c = b, c, cur
    return cur


def main():
    N = 4
    print(tilings(N))  # 11


if __name__ == "__main__":
    main()
