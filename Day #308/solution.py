# Day 308: Count parenthesizations evaluating True. Interval DP. O(n^3) time, O(n^2) space.
def count_true(e):
    n = len(e)
    V = (n + 1) // 2
    T = [[0] * V for _ in range(V)]
    F = [[0] * V for _ in range(V)]
    for i in range(V):
        val = e[2 * i] == "T"
        T[i][i] = 1 if val else 0
        F[i][i] = 0 if val else 1
    for length in range(2, V + 1):
        for i in range(0, V - length + 1):
            j = i + length - 1
            for k in range(i, j):
                op = e[2 * k + 1]
                lt, lf = T[i][k], F[i][k]
                rt, rf = T[k + 1][j], F[k + 1][j]
                total = (lt + lf) * (rt + rf)
                if op == "&":
                    t = lt * rt
                elif op == "|":
                    t = lt * rt + lt * rf + lf * rt
                else:  # ^
                    t = lt * rf + lf * rt
                T[i][j] += t
                F[i][j] += total - t
    return T[0][V - 1]


if __name__ == "__main__":
    print(count_true(["F", "|", "T", "&", "T"]))  # 2
