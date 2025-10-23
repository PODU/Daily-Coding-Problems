# Day 481: Evaluate Reverse Polish Notation using a stack.
# Approach: push operands; on operator pop two and apply. Time O(n), Space O(n).
import operator


def eval_rpn(tokens):
    ops = {
        '+': operator.add,
        '-': operator.sub,
        '*': operator.mul,
        '/': lambda a, b: int(a / b),  # truncate toward zero
    }
    stack = []
    for t in tokens:
        if t in ops:
            b = stack.pop()
            a = stack.pop()
            stack.append(ops[t](a, b))
        else:
            stack.append(int(t))
    return stack.pop()


if __name__ == "__main__":
    tokens = [15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-']
    print(eval_rpn(tokens))  # 5
