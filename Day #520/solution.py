# Day 520: Tree depth = max nesting level of '(' in the representation. O(n) time, O(1) space.
def tree_depth(s):
    depth = best = 0
    for c in s:
        if c == '(':
            depth += 1
            best = max(best, depth)
        elif c == ')':
            depth -= 1
    return best


if __name__ == "__main__":
    print(tree_depth("((((00)0)0)0)"))  # 4
