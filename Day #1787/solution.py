# Day 1787: Josephus problem: iterative recurrence J(n)=(J(n-1)+k)%n, plus k=2 closed form.
# Time O(N) (O(log N) for k=2 closed form), Space O(1).

def josephus(n, k):
    r = 0
    for i in range(2, n + 1):
        r = (r + k) % i
    return r + 1

def josephus2(n):  # k==2 closed form
    p = 1
    while p * 2 <= n:
        p *= 2
    return 2 * (n - p) + 1

if __name__ == "__main__":
    n, k = 5, 2
    ans = josephus(n, k)
    if k == 2:
        ans = josephus2(n)
    print(ans)
