# Day 371: Solve a system of addition-only equations over variables/constants.
# Build a linear system A x = b and run Gauss-Jordan elimination; unique integer
# solution -> mapping, otherwise None. Time O(eqs * vars^2).


def solve(text):
    eqs, varset = [], set()
    for line in text.strip().splitlines():
        line = line.strip()
        if not line:
            continue
        lhs, rhs = line.split("=")
        coeffs, b = {}, 0
        for tok in lhs.split("+"):
            t = tok.strip()
            if t.lstrip("-").isdigit():
                b -= int(t)
            else:
                coeffs[t] = coeffs.get(t, 0) + 1
                varset.add(t)
        for tok in rhs.split("+"):
            t = tok.strip()
            if t.lstrip("-").isdigit():
                b += int(t)
            else:
                coeffs[t] = coeffs.get(t, 0) - 1
                varset.add(t)
        eqs.append((coeffs, b))

    vars_ = sorted(varset)
    idx = {v: i for i, v in enumerate(vars_)}
    n = len(vars_)
    aug = []
    for coeffs, b in eqs:
        row = [0.0] * (n + 1)
        for v, c in coeffs.items():
            row[idx[v]] += c
        row[n] = b
        aug.append(row)

    m = len(aug)
    pivot_cols = []
    pr = 0
    for col in range(n):
        sel = next((r for r in range(pr, m) if abs(aug[r][col]) > 1e-9), -1)
        if sel == -1:
            continue
        aug[pr], aug[sel] = aug[sel], aug[pr]
        pv = aug[pr][col]
        aug[pr] = [x / pv for x in aug[pr]]
        for r in range(m):
            if r != pr and abs(aug[r][col]) > 1e-9:
                f = aug[r][col]
                aug[r] = [a - f * b_ for a, b_ in zip(aug[r], aug[pr])]
        pivot_cols.append(col)
        pr += 1

    for r in range(m):
        if all(abs(aug[r][c]) < 1e-9 for c in range(n)) and abs(aug[r][n]) > 1e-9:
            return None
    if len(pivot_cols) < n:
        return None
    return {vars_[col]: round(aug[i][n]) for i, col in enumerate(pivot_cols)}


def fmt(sol):
    if sol is None:
        return "null"
    lines = ",\n".join(f"  {k}: {sol[k]}" for k in sorted(sol))
    return "{\n" + lines + "\n}"


if __name__ == "__main__":
    text = "y = x + 1\n5 = x + 3\n10 = z + y + 2"
    print(fmt(solve(text)))
