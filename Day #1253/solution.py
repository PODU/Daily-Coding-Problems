# Day 1253: Balanced brackets via stack: push openers, match closers to top. O(n) time, O(n) space.
def is_balanced(s: str) -> bool:
    st = []
    pair = {')': '(', ']': '[', '}': '{'}
    for c in s:
        if c in '([{':
            st.append(c)
        elif c in pair:
            if not st or st.pop() != pair[c]:
                return False
    return not st


if __name__ == "__main__":
    print(str(is_balanced("([])[]({})")).lower())
    print(str(is_balanced("([)]")).lower())
