# Day 676: Valid number validator via single-pass state machine (sign/int/dot/frac/exp).
# Time: O(n) over string length, Space: O(1).

def is_valid_number(s: str) -> bool:
    i, n = 0, len(s)
    if n == 0:
        return False
    if s[i] in '+-':
        i += 1
    digits_before = digits_after = False
    while i < n and s[i].isdigit():
        i += 1
        digits_before = True
    if i < n and s[i] == '.':
        i += 1
        while i < n and s[i].isdigit():
            i += 1
            digits_after = True
    if not digits_before and not digits_after:
        return False
    if i < n and s[i] in 'eE':
        i += 1
        if i < n and s[i] in '+-':
            i += 1
        exp_digits = False
        while i < n and s[i].isdigit():
            i += 1
            exp_digits = True
        if not exp_digits:
            return False
    return i == n


def main():
    tests = ["10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"]
    for t in tests:
        print(f'"{t}" -> {"True" if is_valid_number(t) else "False"}')


if __name__ == "__main__":
    main()
