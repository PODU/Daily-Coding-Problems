# Day 1140: De Bruijn B(k,n) via FKM (Lyndon word) algorithm -> lexicographically smallest. O(k^n).
def de_bruijn(k, n):
    a = [0] * (k * n + 1)
    seq = []

    def db(t, p):
        if t > n:
            if n % p == 0:
                seq.extend(a[1:p + 1])
        else:
            a[t] = a[t - p]
            db(t + 1, p)
            for j in range(a[t - p] + 1, k):
                a[t] = j
                db(t + 1, t)

    db(1, 1)
    return "".join(str(x) for x in seq)


if __name__ == "__main__":
    # C = {0,1} -> alphabet size 2, substring length 3
    print(de_bruijn(2, 3))
