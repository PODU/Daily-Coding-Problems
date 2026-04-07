# Day 1319: Square root of a real number via Newton's method: x <- (x + n/x)/2.
# Quadratic convergence -> O(log(1/eps)) iterations, O(1) space.

def my_sqrt(n):
    if n < 0:
        raise ValueError("negative")
    if n == 0:
        return 0.0
    x = float(n)
    for _ in range(100):
        nx = 0.5 * (x + n / x)
        if abs(nx - x) < 1e-15:
            break
        x = nx
    return x


if __name__ == "__main__":
    r = my_sqrt(9)
    print(round(r) if abs(r - round(r)) < 1e-9 else r)  # 3
