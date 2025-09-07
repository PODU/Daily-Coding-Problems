# Day 233: Fibonacci in O(1) space: iterate keeping only the last two values.
# Time: O(n), Space: O(1).


def fib(n):
    if n < 2:
        return n
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


if __name__ == "__main__":
    print(fib(10))  # 55
