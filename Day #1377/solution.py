# Day 1377: Balanced parens with '*' wildcard: greedy track [lo,hi] of possible open counts.
# Time O(n), Space O(1).


def is_valid(s):
    lo = hi = 0
    for c in s:
        if c == "(":
            lo += 1
            hi += 1
        elif c == ")":
            lo -= 1
            hi -= 1
        else:  # '*' as ')', '(', or empty
            lo -= 1
            hi += 1
        if hi < 0:
            return False
        if lo < 0:
            lo = 0
    return lo == 0


if __name__ == "__main__":
    tests = ["(()*", "(*)", ")*("]
    bal = [s for s in tests if is_valid(s)]
    not_bal = [s for s in tests if not is_valid(s)]
    print(" and ".join(bal) + " are balanced. " + " and ".join(not_bal) + " is not balanced.")
