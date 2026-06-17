# Day 1681: Min parentheses to remove for validity. Track unmatched '(' and count
# unmatched ')'; answer = leftover open + unmatched close. Time O(n), Space O(1).


def min_removals(s):
    open_cnt = removals = 0
    for c in s:
        if c == '(':
            open_cnt += 1
        elif c == ')':
            if open_cnt > 0:
                open_cnt -= 1
            else:
                removals += 1
    return removals + open_cnt


if __name__ == "__main__":
    print(min_removals("()())()"))  # 1
    print(min_removals(")("))        # 2
