// Valid number validator via single-pass state machine (sign/int/dot/frac/exp).
// Time: O(n) over string length, Space: O(1).
'use strict';

function isDigit(c) { return c >= '0' && c <= '9'; }

function isValidNumber(s) {
    let i = 0;
    const n = s.length;
    if (n === 0) return false;
    if (s[i] === '+' || s[i] === '-') i++;
    let digitsBefore = false, digitsAfter = false;
    while (i < n && isDigit(s[i])) { i++; digitsBefore = true; }
    if (i < n && s[i] === '.') {
        i++;
        while (i < n && isDigit(s[i])) { i++; digitsAfter = true; }
    }
    if (!digitsBefore && !digitsAfter) return false;
    if (i < n && (s[i] === 'e' || s[i] === 'E')) {
        i++;
        if (i < n && (s[i] === '+' || s[i] === '-')) i++;
        let expDigits = false;
        while (i < n && isDigit(s[i])) { i++; expDigits = true; }
        if (!expDigits) return false;
    }
    return i === n;
}

function main() {
    const tests = ["10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"];
    for (const t of tests)
        console.log(`"${t}" -> ${isValidNumber(t) ? "True" : "False"}`);
}

main();
