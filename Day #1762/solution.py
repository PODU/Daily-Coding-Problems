# Day 1762: Count parenthesizations of a boolean expression evaluating to True.
# Interval DP over operands: t[i][j]/f[i][j] = #ways range evals True/False,
# combine across each split operator. Time O(n^3), Space O(n^2).
def count_true(tokens):
    vals = [tokens[i] for i in range(0, len(tokens), 2)]
    ops = [tokens[i] for i in range(1, len(tokens), 2)]
    n = len(vals)
    t = [[0] * n for _ in range(n)]
    f = [[0] * n for _ in range(n)]
    for i in range(n):
        t[i][i] = 1 if vals[i] == 'T' else 0
        f[i][i] = 1 if vals[i] == 'F' else 0
    for length in range(2, n + 1):
        for i in range(0, n - length + 1):
            j = i + length - 1
            for k in range(i, j):
                op = ops[k]
                lt, lf, rt, rf = t[i][k], f[i][k], t[k + 1][j], f[k + 1][j]
                tot = (lt + lf) * (rt + rf)
                if op == '&':
                    t[i][j] += lt * rt
                    f[i][j] += tot - lt * rt
                elif op == '|':
                    f[i][j] += lf * rf
                    t[i][j] += tot - lf * rf
                else:  # ^
                    t[i][j] += lt * rf + lf * rt
                    f[i][j] += lt * rt + lf * rf
    return t[0][n - 1]


if __name__ == "__main__":
    print(count_true(['F', '|', 'T', '&', 'T']))
