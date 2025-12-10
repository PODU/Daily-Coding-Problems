# Day 730: Python closure-in-a-loop late binding.
# The buggy version prints 3,3,3 because every closure captures the SAME variable i,
# whose final value is 3. Fix: bind i per-iteration via a default argument.

def make_functions_buggy():
    flist = []
    for i in [1, 2, 3]:
        def print_i():
            print(i)          # late binding: looks up i when CALLED (i == 3 by then)
        flist.append(print_i)
    return flist


def make_functions_fixed():
    flist = []
    for i in [1, 2, 3]:
        def print_i(i=i):     # capture current value of i now
            print(i)
        flist.append(print_i)
    return flist


if __name__ == "__main__":
    for f in make_functions_buggy():   # prints 3, 3, 3
        f()
    for f in make_functions_fixed():   # prints 1, 2, 3
        f()
