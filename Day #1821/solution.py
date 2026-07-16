# Day 1821: Square root via Newton's method. ~O(log(1/eps)) iterations, O(1) space.
def mysqrt(n, eps=1e-15):
    if n < 0:
        raise ValueError("negative")
    if n == 0:
        return 0.0
    x = float(n)
    for _ in range(200):
        nx = 0.5 * (x + n / x)
        if abs(nx - x) < eps:
            x = nx
            break
        x = nx
    return x


if __name__ == "__main__":
    n = 9
    r = mysqrt(n)
    print(int(round(r)) if abs(r - round(r)) < 1e-9 else r)
