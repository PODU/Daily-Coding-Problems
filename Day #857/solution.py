# Day 857: Depth of tree from (lr) string representation.
# Approach: depth equals the maximum nesting level of parentheses.
# Time: O(n), Space: O(1).


def depth(s):
    cur = mx = 0
    for c in s:
        if c == '(':
            cur += 1
            mx = max(mx, cur)
        elif c == ')':
            cur -= 1
    return mx


if __name__ == "__main__":
    print(depth("(00)"))            # 1
    print(depth("((00)(00))"))      # 2
    print(depth("((((00)0)0)0)"))   # 4
