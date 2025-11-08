# Day 567: Closure-based pair: cons returns a function applying a selector to (a,b); car/cdr pick. Time O(1), Space O(1).
def cons(a, b):
    return lambda f: f(a, b)


def car(pair):
    return pair(lambda a, b: a)


def cdr(pair):
    return pair(lambda a, b: b)


if __name__ == "__main__":
    print(car(cons(3, 4)))
    print(cdr(cons(3, 4)))
