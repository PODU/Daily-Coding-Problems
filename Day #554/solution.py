# Day 554: Egyptian fraction via greedy (Fibonacci/Sylvester): take ceil(b/a) each step.
# Time: O(number of terms), Space: O(1).
def egyptian(a, b):
    terms = []
    while a != 0:
        x = -(-b // a)  # ceil(b/a)
        terms.append(f"1 / {x}")
        a = a * x - b
        b = b * x
    return " + ".join(terms)


if __name__ == "__main__":
    print(egyptian(4, 13))
