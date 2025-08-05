# Day 70: n-th perfect number (digits sum to 10): count up from 1 checking digit sums. Time O(answer * digits), Space O(1).
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
    print(nth_perfect(2))
