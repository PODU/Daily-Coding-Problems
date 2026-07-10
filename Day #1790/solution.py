# Day 1790: fib(n): iterative two-variable rolling sum.
# Time O(N), Space O(1).

def fib(n):
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a

if __name__ == "__main__":
    print(fib(10))
