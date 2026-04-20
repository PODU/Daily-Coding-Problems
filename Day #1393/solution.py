# Day 1393: Josephus problem via iterative recurrence j(i)=(j(i-1)+k)%i, answer j(N)+1. O(N) time, O(1) space.


def josephus(n, k):
    res = 0
    for i in range(2, n + 1):
        res = (res + k) % i
    return res + 1


def main():
    print(josephus(5, 2))


if __name__ == "__main__":
    main()
