# Day 27: Balanced brackets via stack. Time O(n), Space O(n).
def is_balanced(s: str) -> bool:
    stack = []
    match = {')': '(', ']': '[', '}': '{'}
    for c in s:
        if c in '([{':
            stack.append(c)
        elif c in match:
            if not stack or stack.pop() != match[c]:
                return False
    return not stack


if __name__ == "__main__":
    assert is_balanced("([)]") is False
    assert is_balanced("((()") is False
    print("true" if is_balanced("([])[]({})") else "false")
