# Day 1473: Count occurrences of X in an N x N multiplication table.
# For each row i in 1..N, X appears iff i divides X and X//i is within 1..N.
# Time O(N), Space O(1).

def count_x(n, x):
    count = 0
    for i in range(1, n + 1):
        if x % i == 0 and 1 <= x // i <= n:
            count += 1
    return count


if __name__ == "__main__":
    print(count_x(6, 12))  # 4
