# Day 809: Check balanced round/curly/square brackets using a stack.
# Push opens, match closes against stack top. Time O(N), Space O(N).


def is_balanced(s):
    stack = []
    match = {")": "(", "]": "[", "}": "{"}
    for c in s:
        if c in "([{":
            stack.append(c)
        elif c in match:
            if not stack or stack.pop() != match[c]:
                return False
    return not stack


if __name__ == "__main__":
    print(str(is_balanced("([])[]({})")).lower())  # true
    print(str(is_balanced("([)]")).lower())          # false
    print(str(is_balanced("((()")).lower())          # false
