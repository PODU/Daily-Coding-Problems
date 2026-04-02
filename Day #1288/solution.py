# Day 1288: Balance parentheses with minimum insertions+deletions (insertion-only is minimal).
# Single scan: pair each ')' with an open, else insert '(' before it; close leftover opens. O(n).
def balance(s):
    res = []
    open_count = 0
    for ch in s:
        if ch == "(":
            res.append("(")
            open_count += 1
        else:
            if open_count > 0:
                res.append(")")
                open_count -= 1
            else:
                res.append("()")  # insert matching '(' before unmatched ')'
    res.append(")" * open_count)
    return "".join(res)


if __name__ == "__main__":
    print(balance("(()"))    # (())
    print(balance("))()("))  # ()()()()
