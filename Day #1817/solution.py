# Day 1817: De Bruijn sequence B(k_alphabet, n) via the FKM/Lyndon-word recursion.
# Concatenating Lyndon words whose length divides n. Time: O(k^n). Space: O(k^n).


def de_bruijn(C, n):
    C = list(C)
    k = len(C)
    a = [0] * (k * n)
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
    return "".join(C[i] for i in seq)


if __name__ == "__main__":
    print(de_bruijn(["0", "1"], 3))  # 00010111
