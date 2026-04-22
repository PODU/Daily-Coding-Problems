# Day 1405: Interval DP: T[i][j]/F[i][j] = #ways subexpr of operands i..j is True/False.
# Split at each operator, combine child counts per &,|,^. Time O(n^3), Space O(n^2).

def count_true(expr):
    vals = [expr[i] for i in range(0, len(expr), 2)]
    ops = [expr[i] for i in range(1, len(expr), 2)]
    n = len(vals)
    if n == 0:
        return 0
    T = [[0] * n for _ in range(n)]
    F = [[0] * n for _ in range(n)]
    for i in range(n):
        T[i][i] = 1 if vals[i] == 'T' else 0
        F[i][i] = 1 if vals[i] == 'F' else 0
    for length in range(2, n + 1):
        for i in range(0, n - length + 1):
            j = i + length - 1
            for k in range(i, j):
                op = ops[k]
                lt, lf, rt, rf = T[i][k], F[i][k], T[k + 1][j], F[k + 1][j]
                tot = (lt + lf) * (rt + rf)
                if op == '&':
                    t = lt * rt
                elif op == '|':
                    t = lt * rt + lt * rf + lf * rt
                else:  # ^
                    t = lt * rf + lf * rt
                T[i][j] += t
                F[i][j] += tot - t
    return T[0][n - 1]


if __name__ == "__main__":
    print(count_true(['F', '|', 'T', '&', 'T']))  # 2
