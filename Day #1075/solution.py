# Day 1075: Iterative Fibonacci with two rolling variables. O(n) time, O(1) space.
def fib(n):
    if n == 0: return 0
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b

print(" ".join(str(fib(i)) for i in range(11)))
print(f"fib(10) = {fib(10)}")
