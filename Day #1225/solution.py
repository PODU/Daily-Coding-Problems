# Day 1225: Min parens to remove for validity: single pass counting unmatched.
# Time O(n), Space O(1).


def min_removals(s: str) -> int:
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
    print(min_removals("()())()"))
    print(min_removals(")("))
