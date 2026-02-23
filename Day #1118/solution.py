# Day 1118: Day 1118 - Late-binding closure pitfall
# The lambdas all close over the SAME variable i, which equals 9 after the loop,
# so every call prints 9. Fix: bind the current value per iteration (default arg).

def buggy():
    functions = []
    for i in range(10):
        functions.append(lambda: i)   # captures variable i, not its value
    return functions


def fixed():
    functions = []
    for i in range(10):
        functions.append(lambda i=i: i)  # default arg binds current value
    return functions


if __name__ == "__main__":
    print("Buggy output (all 9):")
    for f in buggy():
        print(f())
    print("Fixed output (0-9):")
    for f in fixed():
        print(f())
