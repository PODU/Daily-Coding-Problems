# Day 873: Evaluate Reverse Polish Notation with a stack. Time O(n), Space O(n).
import math


def eval_rpn(tokens):
    st = []
    ops = {"+", "-", "*", "/"}
    for t in tokens:
        if t in ops:
            b = st.pop()
            a = st.pop()
            if t == "+":
                st.append(a + b)
            elif t == "-":
                st.append(a - b)
            elif t == "*":
                st.append(a * b)
            else:
                # truncate toward zero
                st.append(int(a / b) if b != 0 else 0)
        else:
            st.append(int(t))
    return st[-1]


if __name__ == "__main__":
    tokens = [15, 7, 1, 1, "+", "-", "/", 3, "*", 2, 1, 1, "+", "+", "-"]
    print(eval_rpn([str(t) for t in tokens]))
