# Day 1219: Closure late-binding: lambdas capturing loop var see final value; fix = bind per-iteration
# via default arg. O(n) build/call, O(n) space.

if __name__ == "__main__":
    # Buggy: every lambda closes over the same `i`, which ends at 9.
    buggy = []
    for i in range(10):
        buggy.append(lambda: i)
    for f in buggy:
        print(f())

    print("---")

    # Fixed: default argument captures the current value of i each iteration.
    fixed = []
    for i in range(10):
        fixed.append(lambda i=i: i)
    for f in fixed:
        print(f())
