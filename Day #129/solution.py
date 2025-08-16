# Day 129: Square root of a real number via Newton's method.
# Quadratic convergence: O(log(1/eps)) iterations.


def my_sqrt(n):
    if n < 0:
        raise ValueError("negative")
    if n == 0:
        return 0.0
    x = float(n)
    for _ in range(100):
        nx = 0.5 * (x + n / x)
        if abs(nx - x) < 1e-12:
            break
        x = nx
    return x


if __name__ == "__main__":
    n = 9
    r = my_sqrt(n)
    if abs(r - round(r)) < 1e-9:
        print(int(round(r)))
    else:
        print(r)
