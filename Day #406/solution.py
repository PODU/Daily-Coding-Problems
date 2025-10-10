# Day 406: Boolean parenthesization: count ways the expression evaluates to True.
# Interval DP storing #True/#False per substring. Time O(n^3), Space O(n^2).


def count_true(a):
    val = [a[i] for i in range(0, len(a), 2)]
    op = [a[i] for i in range(1, len(a), 2)]
    M = len(val)
    T = [[0] * M for _ in range(M)]
    F = [[0] * M for _ in range(M)]
    for i in range(M):
        T[i][i] = 1 if val[i] == 'T' else 0
        F[i][i] = 1 if val[i] == 'F' else 0
    for length in range(2, M + 1):
        for i in range(0, M - length + 1):
            j = i + length - 1
            for k in range(i, j):
                o = op[k]
                lt, lf, rt, rf = T[i][k], F[i][k], T[k + 1][j], F[k + 1][j]
                tot = (lt + lf) * (rt + rf)
                if o == '&':
                    T[i][j] += lt * rt
                    F[i][j] += tot - lt * rt
                elif o == '|':
                    T[i][j] += tot - lf * rf
                    F[i][j] += lf * rf
                else:
                    T[i][j] += lt * rf + lf * rt
                    F[i][j] += lt * rt + lf * rf
    return T[0][M - 1]


if __name__ == "__main__":
    print(count_true(['F', '|', 'T', '&', 'T']))
