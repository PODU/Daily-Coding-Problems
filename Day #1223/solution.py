# Day 1223: Single linear scan; depth = max paren nesting ('(' +1, ')' -1).
# O(n) time, O(1) space.


def tree_depth(s: str) -> int:
    depth = cur = 0
    for c in s:
        if c == "(":
            cur += 1
            depth = max(depth, cur)
        elif c == ")":
            cur -= 1
    return depth


if __name__ == "__main__":
    print(tree_depth("((((00)0)0)0)"))
