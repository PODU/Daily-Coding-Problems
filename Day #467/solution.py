# Day 467: Square root via Newton's method: x = (x + n/x)/2 until convergence.
# Time: O(log(1/eps)) iterations, Space: O(1).
def my_sqrt(n):
    if n < 0:
        return float("nan")
    if n == 0:
        return 0.0
    x = n
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
        print(round(r))
    else:
        print(r)
