# Day 1282: n-th positive integer whose digits sum to 10.
# Such numbers are ~ every 9th integer; iterate counting matches. Time O(answer/9), Space O(1).
def nth_perfect(n):
    x = 0
    count = 0
    while count < n:
        x += 1
        if sum(int(d) for d in str(x)) == 10:
            count += 1
    return x


if __name__ == "__main__":
    print(nth_perfect(1))  # 19
    print(nth_perfect(2))  # 28
