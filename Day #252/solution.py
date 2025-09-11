# Day 252: Egyptian fraction via greedy: repeatedly take ceil(b/a) as next unit denominator.
# Time: O(number of terms) iterations; Space: O(terms). a/b proper (a<b).

def egyptian(a, b):
    denoms = []
    while a != 0:
        d = -(-b // a)  # ceil(b/a)
        denoms.append(d)
        a = a * d - b
        b = b * d
    return denoms


if __name__ == "__main__":
    denoms = egyptian(4, 13)
    print(" + ".join(f"1 / {d}" for d in denoms))
