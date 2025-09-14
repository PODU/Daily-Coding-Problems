// Day 274: Evaluate string of (), single digits, +/- without eval.
// Stack-based sign tracking. Time O(N), Space O(N).
function evaluate(s) {
  let result = 0, sign = 1;
  const st = [];
  for (const c of s) {
    if (c >= '0' && c <= '9') {
      result += sign * (c.charCodeAt(0) - 48);
    } else if (c === '+') {
      sign = 1;
    } else if (c === '-') {
      sign = -1;
    } else if (c === '(') {
      st.push(result);
      st.push(sign);
      result = 0;
      sign = 1;
    } else if (c === ')') {
      const s2 = st.pop();
      const prev = st.pop();
      result = prev + s2 * result;
    }
  }
  return result;
}

console.log(evaluate("-1 + (2 + 3)")); // 4
