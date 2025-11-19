# Day 624: Minimum parentheses to remove to make string valid: single pass counting unmatched
# open/close. Answer = removals (unmatched ')') + leftover open. Time O(n), Space O(1).


def min_remove(s):
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
    print(min_remove("()())()"))  # 1
    print(min_remove(")("))        # 2
