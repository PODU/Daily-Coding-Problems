# Day 1559: Scan parenthesis string, track open-paren depth, record maximum. Time O(n), Space O(1).

def tree_depth(s):
    depth = max_depth = 0
    for c in s:
        if c == '(':
            depth += 1
            max_depth = max(max_depth, depth)
        elif c == ')':
            depth -= 1
    return max_depth


if __name__ == "__main__":
    print(tree_depth("((((00)0)0)0)"))
