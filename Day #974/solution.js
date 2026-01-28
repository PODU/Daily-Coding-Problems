// Day 974: Evaluate expression with parentheses, digits, +/- (no eval).
// Approach: single scan with a sign/result stack. Time O(n), Space O(n).

function evaluate(s) {
  let result = 0, num = 0, sign = 1;
  const stack = [];
  for (const c of s) {
    if (c >= '0' && c <= '9') {
      num = num * 10 + (c.charCodeAt(0) - 48);
    } else if (c === '+') {
      result += sign * num; num = 0; sign = 1;
    } else if (c === '-') {
      result += sign * num; num = 0; sign = -1;
    } else if (c === '(') {
      stack.push(result); stack.push(sign);
      result = 0; sign = 1;
    } else if (c === ')') {
      result += sign * num; num = 0;
      const prevSign = stack.pop();
      const prevResult = stack.pop();
      result = prevResult + prevSign * result;
      sign = 1;
    }
  }
  result += sign * num;
  return result;
}

console.log(evaluate('-1 + (2 + 3)')); // 4
