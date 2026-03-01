# Day 1137: cons returns a closure pair(f)=f(a,b); car/cdr apply a selector. O(1).
def cons(a, b):
    return lambda f: f(a, b)


def car(pair):
    return pair(lambda a, b: a)


def cdr(pair):
    return pair(lambda a, b: b)


if __name__ == "__main__":
    print(car(cons(3, 4)))
    print(cdr(cons(3, 4)))
