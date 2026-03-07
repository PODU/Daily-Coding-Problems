// Day 1174: Decide whether a string is a valid number.
// Single linear scan: optional sign, integer/fraction digits, optional exponent.
// Time O(N), Space O(1).

function isNumber(s) {
    const n = s.length;
    let i = 0;
    if (n === 0) return false;
    const digit = (k) => k < n && s[k] >= '0' && s[k] <= '9';
    if (i < n && (s[i] === '+' || s[i] === '-')) i++;
    let before = false, after = false;
    while (digit(i)) { i++; before = true; }
    if (i < n && s[i] === '.') {
        i++;
        while (digit(i)) { i++; after = true; }
    }
    if (!before && !after) return false;
    if (i < n && (s[i] === 'e' || s[i] === 'E')) {
        i++;
        if (i < n && (s[i] === '+' || s[i] === '-')) i++;
        let exp = false;
        while (digit(i)) { i++; exp = true; }
        if (!exp) return false;
    }
    return i === n;
}

const tests = ["10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"];
for (const t of tests)
    console.log(`"${t}" -> ${isNumber(t) ? "true" : "false"}`);
