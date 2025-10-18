# Day 450: Balanced parentheses with '*' wildcards via greedy low/high open
# count. O(n) time, O(1) space.


def is_balanced(s):
    low = high = 0  # range of possible open-paren counts
    for c in s:
        if c == '(':
            low += 1
            high += 1
        elif c == ')':
            low -= 1
            high -= 1
        else:  # '*' -> '(', ')', or ''
            low -= 1
            high += 1
        if high < 0:
            return False
        if low < 0:
            low = 0
    return low == 0


if __name__ == "__main__":
    s = "(()*"
    print("balanced" if is_balanced(s) else "not balanced")  # balanced
