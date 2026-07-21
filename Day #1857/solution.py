# Day 1857: Evaluate Reverse Polish Notation.
# Stack: push numbers, pop two on operator and apply. O(n) time, O(n) space.


def eval_rpn(tokens):
    ops = {
        "+": lambda a, b: a + b,
        "-": lambda a, b: a - b,
        "*": lambda a, b: a * b,
        # truncate toward zero (not Python floor division)
        "/": lambda a, b: int(a / b),
    }
    st = []
    for t in tokens:
        if t in ops:
            b = st.pop()
            a = st.pop()
            st.append(ops[t](a, b))
        else:
            st.append(int(t))
    return st[-1]


if __name__ == "__main__":
    tokens = [15, 7, 1, 1, "+", "-", "/", 3, "*", 2, 1, 1, "+", "+", "-"]
    print(eval_rpn(tokens))  # 5
