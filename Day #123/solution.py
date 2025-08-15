# Day 123: Validate whether a string is a number (int/real/scientific).
# Single linear scan state machine. O(n) time, O(1) space.


def is_number(s):
    i, n = 0, len(s)
    if n == 0:
        return False
    if s[i] in "+-":
        i += 1
    digits = dots = 0
    while i < n and (s[i].isdigit() or s[i] == "."):
        if s[i] == ".":
            dots += 1
        else:
            digits += 1
        i += 1
    if dots > 1 or digits == 0:
        return False
    if i < n and s[i] in "eE":
        i += 1
        if i < n and s[i] in "+-":
            i += 1
        expd = 0
        while i < n and s[i].isdigit():
            expd += 1
            i += 1
        if expd == 0:
            return False
    return i == n


if __name__ == "__main__":
    tests = ["10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"]
    for t in tests:
        print('"' + t + '" -> ' + ("true" if is_number(t) else "false"))
