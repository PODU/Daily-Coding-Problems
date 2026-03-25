# Day 1262: Closure-in-a-loop (late binding).
# The closures all capture the SAME variable i (Python late-binds), so after the
# loop i == 3 and every call prints 3. Fix: bind i per-function via a default arg.

def make_functions_buggy():
    flist = []
    for i in [1, 2, 3]:
        def print_i():
            print(i)
        flist.append(print_i)
    return flist


def make_functions_fixed():
    flist = []
    for i in [1, 2, 3]:
        def print_i(i=i):   # capture current value of i as a default argument
            print(i)
        flist.append(print_i)
    return flist


if __name__ == "__main__":
    print("Buggy output:")
    for f in make_functions_buggy():
        f()
    print("Fixed output:")
    for f in make_functions_fixed():
        f()
