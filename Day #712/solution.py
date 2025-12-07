# Day 712: Balanced brackets check using a stack. Time O(n), space O(n).

def balanced(s):
    st = []
    match = {')': '(', ']': '[', '}': '{'}
    for c in s:
        if c in '([{':
            st.append(c)
        elif c in match:
            if not st or st.pop() != match[c]:
                return False
    return not st


if __name__ == "__main__":
    print(str(balanced("([])[]({})")).lower())
    print(str(balanced("([)]")).lower())
    print(str(balanced("((()")).lower())
