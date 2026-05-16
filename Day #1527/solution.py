# Day 1527: Count cells equal to X in an N x N multiplication table (cell(i,j)=i*j).
# For each row i, X is at column X/i iff i divides X and 1<=X/i<=N. O(N) time, O(1) space.

def count_cells(n, x):
    count = 0
    for i in range(1, n + 1):
        if x % i == 0:
            j = x // i
            if 1 <= j <= n:
                count += 1
    return count


if __name__ == "__main__":
    print(count_cells(6, 12))  # expected 4
