# Day 142: Balanced parens with '*' wildcard: track range [lo,hi] of possible open counts. O(n) time, O(1) space.


def is_balanced(s):
    lo = hi = 0
    for c in s:
        if c == '(':
            lo += 1; hi += 1
        elif c == ')':
            lo -= 1; hi -= 1
        else:  # '*' as ')', '(' or empty
            lo -= 1; hi += 1
        if hi < 0:
            return False
        if lo < 0:
            lo = 0
    return lo == 0


if __name__ == "__main__":
    a, b, c = "(()*", "(*)", ")*("
    print(f"{a if is_balanced(a) else ''} and {b if is_balanced(b) else ''} are balanced. "
          f"{c if not is_balanced(c) else ''} is not balanced.")
