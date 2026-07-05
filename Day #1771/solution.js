// Day 1771: Stack-based "basic calculator" for +,-,parentheses,single digits,unary sign.
// Single left-to-right pass with a sign/result stack. Time: O(n), Space: O(n).
'use strict';

function evaluate(s) {
    let result = 0;
    let sign = 1;
    const stack = []; // saved [result, sign] at '('
    for (const c of s) {
        if (c >= '0' && c <= '9') {
            result += sign * (c.charCodeAt(0) - 48);
            sign = 1;
        } else if (c === '+') {
            sign = 1;
        } else if (c === '-') {
            sign = -1;
        } else if (c === '(') {
            stack.push([result, sign]);
            result = 0;
            sign = 1;
        } else if (c === ')') {
            const [prevRes, prevSign] = stack.pop();
            result = prevRes + prevSign * result;
            sign = 1;
        }
    }
    return result;
}

console.log(evaluate("-1 + (2 + 3)"));
