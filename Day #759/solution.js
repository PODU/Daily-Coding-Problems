// Day 759: Generate all valid IPv4 addresses from a digit string (backtracking).
// At most 3^3 splits; Time: O(1) practically (bounded), Space: O(#results).
"use strict";

function validOctet(s) {
    if (!s || s.length > 3) return false;
    if (s.length > 1 && s[0] === '0') return false;
    return parseInt(s, 10) <= 255;
}

function restoreIps(s) {
    const res = [];
    function backtrack(start, parts) {
        if (parts.length === 4) {
            if (start === s.length) res.push(parts.join("."));
            return;
        }
        for (let len = 1; len <= 3 && start + len <= s.length; len++) {
            const seg = s.substring(start, start + len);
            if (validOctet(seg)) backtrack(start + len, parts.concat(seg));
        }
    }
    backtrack(0, []);
    return res;
}

const res = restoreIps("2542540123");
console.log("[" + res.map((x) => `'${x}'`).join(", ") + "]");
// ['254.25.40.123', '254.254.0.123']
