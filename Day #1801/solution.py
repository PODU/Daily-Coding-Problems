# Day 1801: Balance parentheses with min insertions/deletions via insertion-based scan.
# Time O(n), Space O(n).

def balance(s):
    result = []
    open_ = 0
    for c in s:
        if c == '(':
            result.append('(')
            open_ += 1
        else:  # ')'
            if open_ == 0:
                result.append('(')
                result.append(')')
            else:
                result.append(')')
                open_ -= 1
    result += [')'] * open_
    return ''.join(result)


if __name__ == "__main__":
    print(balance("(()"))
    print(balance("))()("))
