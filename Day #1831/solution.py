# Day 1831: Smallest positive int not a subset sum of a sorted array. Greedy O(N).
# Keep reachable range [1..res-1]; if a[i] <= res extend, else res is the answer.
def smallest_missing(a):
    res = 1
    for x in a:
        if x > res:
            break
        res += x
    return res


if __name__ == "__main__":
    print(smallest_missing([1, 2, 3, 10]))  # 7
