# Day 1174: Decide whether a string is a valid number.
# Single linear scan: optional sign, integer/fraction digits, optional exponent.
# Time O(N), Space O(1).


def is_number(s):
    n, i = len(s), 0
    if n == 0:
        return False
    if i < n and s[i] in "+-":
        i += 1
    before = after = False
    while i < n and s[i].isdigit():
        i += 1
        before = True
    if i < n and s[i] == '.':
        i += 1
        while i < n and s[i].isdigit():
            i += 1
            after = True
    if not before and not after:
        return False
    if i < n and s[i] in "eE":
        i += 1
        if i < n and s[i] in "+-":
            i += 1
        exp = False
        while i < n and s[i].isdigit():
            i += 1
            exp = True
        if not exp:
            return False
    return i == n


if __name__ == "__main__":
    tests = ["10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"]
    for t in tests:
        print(f'"{t}" -> {"true" if is_number(t) else "false"}')
