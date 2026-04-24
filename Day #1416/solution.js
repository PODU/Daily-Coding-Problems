// Day 1416: evaluate a +/-/parenthesized expression of single digits, no eval.
// Approach: single scan with a sign stack (Basic Calculator). O(n) time, O(n) space.

function evaluate(s) {
  let result = 0, sign = 1, i = 0;
  const n = s.length;
  const stack = [1];
  const isDigit = (ch) => ch >= "0" && ch <= "9";
  while (i < n) {
    const c = s[i];
    if (isDigit(c)) {
      let num = 0;
      while (i < n && isDigit(s[i])) {
        num = num * 10 + (s.charCodeAt(i) - 48);
        i++;
      }
      result += sign * stack[stack.length - 1] * num;
      continue;
    } else if (c === "+") sign = 1;
    else if (c === "-") sign = -1;
    else if (c === "(") { stack.push(sign * stack[stack.length - 1]); sign = 1; }
    else if (c === ")") stack.pop();
    i++;
  }
  return result;
}

console.log(evaluate("-1 + (2 + 3)")); // 4
