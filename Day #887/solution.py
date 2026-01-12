# Day 887: Egyptian fraction via greedy: take ceil(b/a) each step. Time O(terms), Space O(1).
def egyptian(a, b):
    terms = []
    while a != 0:
        x = -(-b // a)  # ceil(b/a)
        terms.append("1 / {}".format(x))
        a, b = a * x - b, b * x
    return " + ".join(terms)


if __name__ == "__main__":
    print(egyptian(4, 13))
