# Day 846: implement car/cdr for closure-based cons.
# cons stores a pair as a function taking a selector; car/cdr pass a selector. O(1).

def cons(a, b):
    def pair(f):
        return f(a, b)
    return pair


def car(pair):
    return pair(lambda a, b: a)


def cdr(pair):
    return pair(lambda a, b: b)


if __name__ == "__main__":
    print(car(cons(3, 4)))  # 3
    print(cdr(cons(3, 4)))  # 4
