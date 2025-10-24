# Day 483: Josephus problem.
# Iterative recurrence f(i)=(f(i-1)+k)%i in O(n) time, O(1) space.
# Special O(log n) closed form when k=2.


def josephus(n, k):
    result = 0  # 0-indexed survivor among 1 person
    for i in range(2, n + 1):
        result = (result + k) % i
    return result + 1  # 1-indexed


def josephus_k2(n):
    # O(log n) special case for k == 2.
    p = 1
    while p * 2 <= n:
        p *= 2
    return 2 * (n - p) + 1


if __name__ == "__main__":
    n, k = 5, 2
    print(josephus(n, k))  # 3
