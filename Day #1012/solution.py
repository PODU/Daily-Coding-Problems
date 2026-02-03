# Day 1012: Square root via Newton's method: x = (x + n/x)/2, quadratic convergence.
# Time: O(log(1/eps)) iterations, Space: O(1).

def my_sqrt(n):
    if n < 0:
        return float('nan')
    if n == 0:
        return 0.0
    x = float(n)
    for _ in range(100):
        nx = 0.5 * (x + n / x)
        if abs(nx - x) < 1e-15:
            x = nx
            break
        x = nx
    return x


def print_result(n):
    r = my_sqrt(n)
    ri = round(r)
    if abs(r - ri) < 1e-9:
        print(ri)              # exact integer
    else:
        print(f"{r:.8f}")


if __name__ == "__main__":
    print_result(9)   # -> 3
    print_result(2)   # -> 1.41421356
