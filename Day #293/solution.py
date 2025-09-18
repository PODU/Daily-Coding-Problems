# Day 293: Pyramid reshape (lowering only): L[i]/R[i] cap ramp slopes, peak v=max min(L,R), cost=sum-v*v.
# Time O(n), Space O(n).
def min_cost(h):
    n = len(h)
    L = [0] * n
    R = [0] * n
    L[0] = min(h[0], 1)
    for i in range(1, n):
        L[i] = min(h[i], L[i-1] + 1)
    R[n-1] = min(h[n-1], 1)
    for i in range(n-2, -1, -1):
        R[i] = min(h[i], R[i+1] + 1)
    v = max(min(L[i], R[i]) for i in range(n))
    return sum(h) - v * v


if __name__ == "__main__":
    h = [1, 1, 3, 3, 2, 1]
    print(min_cost(h))
