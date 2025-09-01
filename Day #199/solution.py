# Day 199: Balance parentheses with minimum insertions/deletions.
# Greedy: each unmatched paren costs exactly 1 op; inserting its partner is always optimal.
# Time: O(n), Space: O(n).


def balance(s):
    res = []
    open_count = 0
    for c in s:
        if c == '(':
            res.append('(')
            open_count += 1
        else:
            if open_count > 0:
                res.append(')')
                open_count -= 1
            else:
                res.append('()')  # unmatched ')': insert a '(' before it
    res.append(')' * open_count)  # close remaining opens
    return ''.join(res)


if __name__ == "__main__":
    print(balance("(()"))    # (())
    print(balance("))()("))  # ()()()()
