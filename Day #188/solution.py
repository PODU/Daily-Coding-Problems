# Day 188: Closure-in-a-loop late binding.
# The closures share the loop variable `i`; by call time the loop has finished so i==3.
# Fix: bind the current value per iteration (default argument). Time/Space O(n).


def make_functions_buggy():
    flist = []
    for i in [1, 2, 3]:
        def print_i():
            return i  # late binding: reads i at call time -> 3
        flist.append(print_i)
    return flist


def make_functions_fixed():
    flist = []
    for i in [1, 2, 3]:
        def print_i(i=i):  # default arg captures current value
            return i
        flist.append(print_i)
    return flist


if __name__ == "__main__":
    print("Late binding prints:")
    for f in make_functions_buggy():
        print(f())
    print("Fixed prints:")
    for f in make_functions_fixed():
        print(f())
