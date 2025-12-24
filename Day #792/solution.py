# Day 792: Pyramid: left[i]=min(h,left[i-1]+1), right[i]=min(h,right[i+1]+1), cap=min(left,right).
# Peak P=max(cap); target descends from P both sides. cost=sum(h)-sum(target). O(n) time/space.
def min_pyramid_cost(h):
    n = len(h)
    left = [0] * n
    right = [0] * n
    left[0] = min(h[0], 1)
    for i in range(1, n):
        left[i] = min(h[i], left[i-1] + 1)
    right[n-1] = min(h[n-1], 1)
    for i in range(n - 2, -1, -1):
        right[i] = min(h[i], right[i+1] + 1)
    cap = [min(left[i], right[i]) for i in range(n)]
    P = max(cap)
    k = cap.index(P)
    target = [0] * n
    target[k] = P
    for j in range(1, k + 1):
        target[k-j] = max(0, P - j)
    for j in range(1, n - k):
        target[k+j] = max(0, P - j)
    cost = sum(h) - sum(target)
    return cost, target


if __name__ == "__main__":
    cost, target = min_pyramid_cost([1, 1, 3, 3, 2, 1])
    print(f"{cost} (resulting in {target})")
