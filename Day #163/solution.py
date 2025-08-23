# Day 163: Evaluate Reverse Polish Notation with a stack. Push operands, on an
# operator pop two and apply. Time O(n), Space O(n).


def eval_rpn(tokens):
    stack = []
    ops = {
        "+": lambda a, b: a + b,
        "-": lambda a, b: a - b,
        "*": lambda a, b: a * b,
        "/": lambda a, b: int(a / b),  # truncate toward zero
    }
    for t in tokens:
        if t in ops:
            b = stack.pop()
            a = stack.pop()
            stack.append(ops[t](a, b))
        else:
            stack.append(int(t))
    return stack.pop()


if __name__ == "__main__":
    tokens = [15, 7, 1, 1, "+", "-", "/", 3, "*", 2, 1, 1, "+", "+", "-"]
    print(eval_rpn(tokens))  # 5
