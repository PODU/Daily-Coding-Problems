# Day 1416: evaluate a +/-/parenthesized expression of single digits, no eval.
# Approach: single scan with a sign stack (Basic Calculator). O(n) time, O(n) space.


def evaluate(s):
    result, sign = 0, 1
    stack = [1]
    i, n = 0, len(s)
    while i < n:
        c = s[i]
        if c.isdigit():
            num = 0
            while i < n and s[i].isdigit():
                num = num * 10 + int(s[i])
                i += 1
            result += sign * stack[-1] * num
            continue
        elif c == "+":
            sign = 1
        elif c == "-":
            sign = -1
        elif c == "(":
            stack.append(sign * stack[-1])
            sign = 1
        elif c == ")":
            stack.pop()
        i += 1
    return result


if __name__ == "__main__":
    print(evaluate("-1 + (2 + 3)"))  # 4
