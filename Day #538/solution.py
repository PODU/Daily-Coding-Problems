# Day 538: De Bruijn sequence via FKM (Lyndon-word/necklace) algorithm: emit Lyndon words whose
# length divides n, in order, giving lexicographically smallest sequence. Time O(k^n).

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
    print(de_bruijn(2, 3))  # 00010111
