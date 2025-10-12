# Day 420: n-th positive integer whose digits sum to exactly 10.
# Iterate integers, count those with digit sum 10. Time: O(answer), Space: O(1).


def digit_sum(x):
    s = 0
    while x > 0:
        s += x % 10
        x //= 10
    return s


def nth_perfect(n):
    count = 0
    x = 0
    while count < n:
        x += 1
        if digit_sum(x) == 10:
            count += 1
    return x


if __name__ == "__main__":
    print(nth_perfect(1))  # 19
    print(nth_perfect(2))  # 28
