# Day 1688: cons(a,b) returns a closure pair(f)=f(a,b); car/cdr pass selectors. All O(1).

def cons(a, b):
    return lambda f: f(a, b)

def car(p):
    return p(lambda a, b: a)

def cdr(p):
    return p(lambda a, b: b)

if __name__ == "__main__":
    print(car(cons(3, 4)))
    print(cdr(cons(3, 4)))
