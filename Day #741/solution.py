# Day 741: Evaluate +/- expression with parentheses using a sign stack (Basic Calculator).
# Single linear scan; parentheses handled by pushing the running result and sign.
# Time: O(n), Space: O(n).

def evaluate(s):
    result = 0
    sign = 1
    stack = []
    i, n = 0, len(s)
    while i < n:
        c = s[i]
        if c.isdigit():
            num = 0
            while i < n and s[i].isdigit():
                num = num * 10 + int(s[i])
                i += 1
            result += sign * num
            continue
        elif c == '+':
            sign = 1
        elif c == '-':
            sign = -1
        elif c == '(':
            stack.append(result)
            stack.append(sign)
            result, sign = 0, 1
        elif c == ')':
            s2 = stack.pop()
            r2 = stack.pop()
            result = r2 + s2 * result
        i += 1
    return result


if __name__ == "__main__":
    print(evaluate("-1 + (2 + 3)"))  # 4
