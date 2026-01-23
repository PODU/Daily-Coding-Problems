# Day 942: Min parentheses to remove to make the string valid.
# One pass: count unmatched ')' immediately + leftover unmatched '('. O(n) time, O(1) space.


def min_removals(s):
    open_count = 0
    removals = 0
    for c in s:
        if c == '(':
            open_count += 1
        elif c == ')':
            if open_count > 0:
                open_count -= 1
            else:
                removals += 1
    return removals + open_count


if __name__ == "__main__":
    print(min_removals("()())()"))  # 1
    print(min_removals(")("))         # 2
