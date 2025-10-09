# Day 399: Partition into 3 contiguous equal-sum parts: greedy prefix cut at target, absorbing trailing zeros. O(n) time, O(n) space.
def partition3(L):
    total = sum(L)
    if total % 3 != 0:
        return None
    target = total // 3
    res = []
    cur = []
    running = 0
    n = len(L)
    for i, x in enumerate(L):
        cur.append(x)
        running += x
        # close part when sum hits target and next element is non-zero (zeros stay attached)
        if len(res) < 2 and running == target and (i + 1 == n or L[i + 1] != 0):
            res.append(cur)
            cur = []
            running = 0
    res.append(cur)
    if len(res) != 3 or any(sum(p) != target for p in res):
        return None
    return res


if __name__ == "__main__":
    L = [3, 5, 8, 0, 8]
    print(partition3(L))
