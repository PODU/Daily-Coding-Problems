# Day 274: Evaluate string of (), single digits, +/- without eval.
# Stack-based sign tracking. Time O(N), Space O(N).


def evaluate(s):
    result, sign = 0, 1
    st = []
    for c in s:
        if c.isdigit():
            result += sign * int(c)
        elif c == '+':
            sign = 1
        elif c == '-':
            sign = -1
        elif c == '(':
            st.append(result)
            st.append(sign)
            result, sign = 0, 1
        elif c == ')':
            s2 = st.pop()
            prev = st.pop()
            result = prev + s2 * result
    return result


if __name__ == "__main__":
    print(evaluate("-1 + (2 + 3)"))  # 4
