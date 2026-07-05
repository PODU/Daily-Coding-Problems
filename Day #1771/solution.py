# Day 1771: Stack-based "basic calculator" for +,-,parentheses,single digits,unary sign.
# Single left-to-right pass with a sign/result stack. Time: O(n), Space: O(n).


def evaluate(s):
    result = 0
    sign = 1
    stack = []  # saved (result, sign) at '('
    for c in s:
        if c.isdigit():
            result += sign * int(c)
            sign = 1
        elif c == '+':
            sign = 1
        elif c == '-':
            sign = -1
        elif c == '(':
            stack.append((result, sign))
            result = 0
            sign = 1
        elif c == ')':
            prev_res, prev_sign = stack.pop()
            result = prev_res + prev_sign * result
            sign = 1
    return result


if __name__ == "__main__":
    print(evaluate("-1 + (2 + 3)"))
