# Day 430: Balance parentheses with the minimum number of insertions + deletions.
# One pass: keep matched pairs; turn each unmatched ')' into "()" and each leftover '(' gets a ')'.
# Time O(n), Space O(n).
def balance(s):
    res = []
    open_ = 0
    for c in s:
        if c == '(':
            open_ += 1
            res.append('(')
        else:  # ')'
            if open_ > 0:
                open_ -= 1
                res.append(')')
            else:
                res.append('(')
                res.append(')')
    res.append(')' * open_)
    return ''.join(res)


if __name__ == "__main__":
    print(balance("(()"))
    print(balance("))()("))
