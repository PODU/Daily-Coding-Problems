# Day 928: Iterate integers, sum digits, count until the n-th whose digit sum is 10.
# Time O(answer * digits), Space O(1).
def nth_perfect(n):
    count = 0
    num = 0
    while count < n:
        num += 1
        if sum(int(d) for d in str(num)) == 10:
            count += 1
    return num


if __name__ == "__main__":
    print(nth_perfect(1))
