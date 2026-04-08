# Day 1323: Min lowering cost to form a pyramid (rise by 1 to a peak, fall by 1, unit ends).
# left[i]=min(a[i],left[i-1]+1), right[i] symmetric; best peak h=max(min(left,right)); pyramid sums to h^2.
# Cost = sum(a) - h^2. O(n) time, O(n) space.

def pyramid(a):
    n = len(a)
    left = [0] * n
    right = [0] * n
    for i in range(n):
        left[i] = min(a[i], (left[i - 1] if i else 0) + 1)
    for i in range(n - 1, -1, -1):
        right[i] = min(a[i], (right[i + 1] if i < n - 1 else 0) + 1)

    h, peak = 0, 0
    for i in range(n):
        hi = min(left[i], right[i])
        if hi > h:
            h, peak = hi, i

    target = [0] * n
    for i in range(n):
        d = abs(i - peak)
        if d < h:
            target[i] = h - d
    cost = sum(a) - h * h
    return cost, target


if __name__ == "__main__":
    cost, target = pyramid([1, 1, 3, 3, 2, 1])
    print(cost)    # 2
    print(target)  # [0, 1, 2, 3, 2, 1]
