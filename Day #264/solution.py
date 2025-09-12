# Day 264: De Bruijn sequence B(k, n) over a character set.
# Approach: Fredricksen-Kessler-Maiorana algorithm — concatenate Lyndon words
# whose length divides n, generated via Duval-style recursion.
# Time O(k^n) (size of the output), Space O(n).


def de_bruijn(alphabet, n):
    """alphabet: list of chars; n: substring length. Returns the cyclic sequence."""
    k = len(alphabet)
    a = [0] * (k * n)
    sequence = []

    def db(t, p):
        if t > n:
            if n % p == 0:
                sequence.extend(a[1:p + 1])
        else:
            a[t] = a[t - p]
            db(t + 1, p)
            for j in range(a[t - p] + 1, k):
                a[t] = j
                db(t + 1, t)

    db(1, 1)
    return "".join(alphabet[i] for i in sequence)


def main():
    # C = {0, 1}, k = 3
    print(de_bruijn(["0", "1"], 3))


if __name__ == "__main__":
    main()
