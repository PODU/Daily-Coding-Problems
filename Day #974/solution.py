# Day 974: Evaluate expression with parentheses, digits, +/- (no eval).
# Approach: single scan with a sign/result stack. Time O(n), Space O(n).


def evaluate(s):
    result = 0
    num = 0
    sign = 1
    stack = []
    for c in s:
        if c.isdigit():
            num = num * 10 + int(c)
        elif c == '+':
            result += sign * num
            num = 0
            sign = 1
        elif c == '-':
            result += sign * num
            num = 0
            sign = -1
        elif c == '(':
            stack.append(result)
            stack.append(sign)
            result = 0
            sign = 1
        elif c == ')':
            result += sign * num
            num = 0
            prev_sign = stack.pop()
            prev_result = stack.pop()
            result = prev_result + prev_sign * result
            sign = 1
    result += sign * num
    return result


if __name__ == '__main__':
    print(evaluate("-1 + (2 + 3)"))  # 4
