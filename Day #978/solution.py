# Day 978: Valid number check via single-pass finite-state parser.
# Grammar: [sign] digits [. digits] | [sign] [digits] . digits, then optional (e/E [sign] digits).
# Time: O(n); Space: O(1).


def is_number(s):
    i, n = 0, len(s)
    if n == 0:
        return False
    if s[i] in "+-":
        i += 1

    digits_before = digits_after = False
    while i < n and s[i].isdigit():
        i += 1
        digits_before = True
    if i < n and s[i] == ".":
        i += 1
        while i < n and s[i].isdigit():
            i += 1
            digits_after = True
    if not digits_before and not digits_after:   # mantissa needs a digit
        return False

    if i < n and s[i] in "eE":
        i += 1
        if i < n and s[i] in "+-":
            i += 1
        exp_digits = False
        while i < n and s[i].isdigit():
            i += 1
            exp_digits = True
        if not exp_digits:                        # exponent needs a digit
            return False
    return i == n                                 # no trailing junk


if __name__ == "__main__":
    valid = ["10", "-10", "10.1", "-10.1", "1e5"]
    invalid = ["a", "x 1", "a -2", "-", "", " "]
    for s in valid + invalid:
        print(f'"{s}" -> {is_number(s)}')
