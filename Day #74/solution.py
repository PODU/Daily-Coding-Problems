# Day 74: Count occurrences of X in N×N table: for each row i (1..N), X appears iff i|X and X/i in [1..N].
# Time O(N), Space O(1).
def count_x(N, X):
    return sum(1 for i in range(1, N + 1) if X % i == 0 and 1 <= X // i <= N)


if __name__ == "__main__":
    print(count_x(6, 12))
