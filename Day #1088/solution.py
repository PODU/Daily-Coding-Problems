# Day 1088: Boolean parenthesization via interval DP counting True/False groupings. Time O(n^3), Space O(n^2).
def count_true(expr):
    m = len(expr)
    n = (m + 1) // 2
    val = [expr[i] == 'T' for i in range(0, m, 2)]
    ops = [expr[i] for i in range(1, m, 2)]
    T = [[0] * n for _ in range(n)]
    F = [[0] * n for _ in range(n)]
    for i in range(n):
        T[i][i] = 1 if val[i] else 0
        F[i][i] = 0 if val[i] else 1
    for length in range(2, n + 1):
        for i in range(n - length + 1):
            j = i + length - 1
            for k in range(i, j):
                op = ops[k]
                lt, lf, rt, rf = T[i][k], F[i][k], T[k + 1][j], F[k + 1][j]
                if op == '&':
                    T[i][j] += lt * rt
                    F[i][j] += lf * rf + lf * rt + lt * rf
                elif op == '|':
                    T[i][j] += lt * rt + lt * rf + lf * rt
                    F[i][j] += lf * rf
                else:
                    T[i][j] += lt * rf + lf * rt
                    F[i][j] += lt * rt + lf * rf
    return T[0][n - 1]


if __name__ == "__main__":
    print(count_true(['F', '|', 'T', '&', 'T']))
