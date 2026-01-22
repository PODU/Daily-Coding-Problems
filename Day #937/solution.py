# Day 937: Valid parenthesis string with '*' (= '(' , ')' or empty).
# Greedy: track [lo,hi] range of possible open counts. Valid iff lo can reach 0. O(n) time, O(1) space.


def is_valid(s):
    lo = hi = 0
    for c in s:
        if c == '(':
            lo += 1
            hi += 1
        elif c == ')':
            lo -= 1
            hi -= 1
        else:  # '*'
            lo -= 1
            hi += 1
        if hi < 0:
            return False
        if lo < 0:
            lo = 0
    return lo == 0


if __name__ == "__main__":
    inputs = ["(()*", "(*)", ")*("]
    bal = [s for s in inputs if is_valid(s)]
    notbal = [s for s in inputs if not is_valid(s)]
    print(f'{" and ".join(bal)} are balanced. {" and ".join(notbal)} is not balanced.')
    # (()* and (*) are balanced. )*( is not balanced.
