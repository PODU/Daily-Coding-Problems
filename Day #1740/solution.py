# Day 1740: Approach: deterministic single linear scan validating sign/digits/dot/exponent.
# Time O(n), Space O(1).
def is_number(s: str) -> bool:
    n = len(s)
    if n == 0:
        return False
    i = 0
    if s[i] in '+-':
        i += 1
    digits = False
    dot = False
    while i < n and (s[i].isdigit() or s[i] == '.'):
        if s[i] == '.':
            if dot:
                return False
            dot = True
        else:
            digits = True
        i += 1
    if not digits:
        return False
    if i < n and s[i] in 'eE':
        i += 1
        if i < n and s[i] in '+-':
            i += 1
        exp_digits = False
        while i < n and s[i].isdigit():
            exp_digits = True
            i += 1
        if not exp_digits:
            return False
    return i == n


if __name__ == "__main__":
    tests = ["10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"]
    for t in tests:
        print("true" if is_number(t) else "false")
