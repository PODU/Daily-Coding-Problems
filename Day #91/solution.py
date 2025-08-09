# Day 91: Closure captures variable, not value. The loop prints 9 ten times
# because all lambdas share the same `i`. Fix: bind i via default arg. O(n).

def make_broken():
    fns = []
    for i in range(10):
        fns.append(lambda: i)  # captures `i` by reference
    return fns


def make_fixed():
    fns = []
    for i in range(10):
        fns.append(lambda i=i: i)  # capture current value via default arg
    return fns


if __name__ == "__main__":
    print("Broken (prints 9 ten times):", [f() for f in make_broken()])
    print("Fixed:", [f() for f in make_fixed()])
