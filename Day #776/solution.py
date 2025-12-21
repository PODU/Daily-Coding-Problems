# Day 776: Josephus problem. General O(N) recurrence j=(j+k)%i.
# For k=2 an O(log N) closed form is also given. Returns 1-indexed survivor.


def josephus(n, k):
    r = 0
    for i in range(2, n + 1):
        r = (r + k) % i
    return r + 1


def josephus_k2(n):  # O(log N)
    p = 1
    while p * 2 <= n:
        p *= 2
    return 2 * (n - p) + 1


if __name__ == "__main__":
    print(josephus(5, 2))    # 3
    print(josephus_k2(5))    # 3
