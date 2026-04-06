# Day 1314: Egyptian fraction via Fibonacci/Sylvester greedy: repeatedly subtract the
# largest unit fraction 1/ceil(b/a). Time O(#terms), Space O(1).

def egyptian(a, b):
    terms = []
    while a != 0:
        x = -(-b // a)  # ceil(b/a)
        terms.append(f"1 / {x}")
        a, b = a * x - b, b * x
    return " + ".join(terms)


if __name__ == "__main__":
    print(egyptian(4, 13))  # 1 / 4 + 1 / 18 + 1 / 468
