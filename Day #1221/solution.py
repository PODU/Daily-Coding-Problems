# Day 1221: Iterative rolling Fibonacci: two variables, O(n) time, O(1) space.
# fib(0)=0, fib(1)=1.


def fib(n: int) -> int:
    if n < 2:
        return n
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


if __name__ == "__main__":
    print(fib(10))
