# Day 86: Min parentheses to remove for validity. Track unmatched '(' and ')'.
# Time O(n), Space O(1).


def min_removal(s):
    open_count = 0
    removals = 0
    for c in s:
        if c == "(":
            open_count += 1
        elif c == ")":
            if open_count > 0:
                open_count -= 1
            else:
                removals += 1  # unmatched ')'
    return removals + open_count  # leftover unmatched '('


if __name__ == "__main__":
    print(min_removal("()())()"))  # 1
    print(min_removal(")("))        # 2
