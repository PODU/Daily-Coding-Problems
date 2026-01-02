// Day 840: Print a string in zigzag form across k lines.
// Char i sits at column i; its row follows the zigzag 0,1,..,k-1,k-2,..,1,0,...
// Build k rows of spaces, place each char, print with trailing spaces trimmed. Time O(N*k).
'use strict';

function zigzag(s, k) {
    if (k <= 0) return "";
    if (k === 1) return s;
    const n = s.length;
    const rows = Array.from({ length: k }, () => new Array(n).fill(' '));
    let row = 0, step = 1;
    for (let i = 0; i < n; i++) {
        rows[row][i] = s[i];
        if (row === 0) step = 1;
        else if (row === k - 1) step = -1;
        row += step;
    }
    return rows.map(r => r.join('').replace(/\s+$/, '')).join('\n');
}

console.log(zigzag("thisisazigzag", 4));
