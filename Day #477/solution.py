# Day 477: Closure late-binding: lambdas capture the variable i (not its value), so all print 9 (its
# final value after the loop). Fix with a per-iteration default arg: lambda i=i: i -> prints 0..9.
def buggy():
    fns = []
    for i in range(10):
        fns.append(lambda: i)        # captures variable i, resolved at call time
    return fns


def fixed():
    fns = []
    for i in range(10):
        fns.append(lambda i=i: i)    # default arg binds the current value of i
    return fns


if __name__ == "__main__":
    print("Buggy:")
    for f in buggy():
        print(f())
    print("Fixed:")
    for f in fixed():
        print(f())
