# Day 557: Count occurrences of X in an N x N multiplication table.
# For each row i (1..N), X appears iff i divides X and X/i is in [1,N]. O(N) time, O(1) space.

def count_x(n, x):
    cnt = 0
    for i in range(1, n + 1):
        if x % i == 0 and 1 <= x // i <= n:
            cnt += 1
    return cnt


if __name__ == "__main__":
    print(count_x(6, 12))  # 4
