// Evaluate +/- expression with parentheses using a sign stack (Basic Calculator).
// Single linear scan; parentheses handled by pushing the running result and sign.
// Time: O(n), Space: O(n).

function evaluate(s) {
  let result = 0, sign = 1;
  const stack = [];
  let i = 0;
  const n = s.length;
  const isDigit = (ch) => ch >= '0' && ch <= '9';
  while (i < n) {
    const c = s[i];
    if (isDigit(c)) {
      let num = 0;
      while (i < n && isDigit(s[i])) { num = num * 10 + (s.charCodeAt(i) - 48); i++; }
      result += sign * num;
      continue;
    } else if (c === '+') { sign = 1; }
    else if (c === '-') { sign = -1; }
    else if (c === '(') { stack.push(result); stack.push(sign); result = 0; sign = 1; }
    else if (c === ')') { const s2 = stack.pop(); const r2 = stack.pop(); result = r2 + s2 * result; }
    i++;
  }
  return result;
}

console.log(evaluate("-1 + (2 + 3)")); // 4
