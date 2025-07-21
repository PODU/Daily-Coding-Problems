# Day 5: Closure-based pair: cons stores (a,b) in a closure; car/cdr apply a selector.
# Time: O(1), Space: O(1) per pair.
def cons(a, b):
    def pair(f):
        return f(a, b)
    return pair


def car(pair):
    return pair(lambda a, b: a)


def cdr(pair):
    return pair(lambda a, b: b)


if __name__ == "__main__":
    print(car(cons(3, 4)))
    print(cdr(cons(3, 4)))
