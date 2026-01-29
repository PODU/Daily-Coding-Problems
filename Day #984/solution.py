# Day 984: Balance a parentheses string with minimum insertions+deletions (insertion-only
# greedy is provably optimal: each unmatched paren needs exactly one edit).
# Time: O(n), Space: O(n).


def balance(s):
    res = []
    bal = 0
    for c in s:
        if c == '(':
            res.append('(')
            bal += 1
        else:  # ')'
            if bal > 0:
                res.append(')')
                bal -= 1
            else:
                res.append('()')  # insert '(' to match this ')'
    res.append(')' * bal)  # close any still-open '('
    return ''.join(res)


if __name__ == "__main__":
    print(balance("(()"))
    print(balance("))()("))
