# Day 850: De Bruijn sequence via the FKM (Lyndon-word) algorithm.
# Lexicographically smallest cyclic sequence containing every length-n string once. O(k^n).

def de_bruijn(k, n, alphabet):
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
    return "".join(alphabet[i] for i in seq)


if __name__ == "__main__":
    # C = {0, 1}, length 3 => alphabet size 2, n = 3
    print(de_bruijn(2, 3, "01"))  # 00010111
