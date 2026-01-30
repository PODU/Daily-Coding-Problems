# Day 991: Python closure late-binding.
# The original prints 3, 3, 3 because each closure captures the variable `i` by
# reference (looked up at call time), not by value at definition time. Fix by
# binding i as a default argument (evaluated at def time). O(n) time/space.

def make_functions_buggy():
    flist = []
    for i in [1, 2, 3]:
        def print_i():
            return i          # late binding: reads i at call time
        flist.append(print_i)
    return flist


def make_functions_fixed():
    flist = []
    for i in [1, 2, 3]:
        def print_i(i=i):     # capture current value of i as default arg
            return i
        flist.append(print_i)
    return flist


if __name__ == "__main__":
    print("Buggy:", [f() for f in make_functions_buggy()])  # [3, 3, 3]
    print("Fixed:", [f() for f in make_functions_fixed()])  # [1, 2, 3]
