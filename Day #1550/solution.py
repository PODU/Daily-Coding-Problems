# Day 1550: Python late-binding closure bug: lambdas capture variable `i` (shared), not its value.
# Buggy loop prints final i (9) ten times; fix binds value via default arg. Time O(n), Space O(n).

def main():
    # Buggy: late binding -> all lambdas read the same `i`, which ends at 9.
    buggy = []
    for i in range(10):
        buggy.append(lambda: i)
    for f in buggy:
        print(f())

    print()

    # Fixed: capture per-iteration value via default argument.
    fixed = []
    for i in range(10):
        fixed.append(lambda i=i: i)
    for f in fixed:
        print(f())


if __name__ == "__main__":
    main()
