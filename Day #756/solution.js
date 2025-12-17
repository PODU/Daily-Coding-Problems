// Day 756: Recover digits from an anagram of their English spellings.
// Use unique marker letters (z,w,u,x,g) then deduce the rest. Time: O(n), Space: O(1).
"use strict";

function recoverDigits(s) {
    const cnt = {};
    for (const c of s) cnt[c] = (cnt[c] || 0) + 1;
    const g = (ch) => cnt[ch] || 0;
    const d = new Array(10).fill(0);
    d[0] = g('z');
    d[2] = g('w');
    d[4] = g('u');
    d[6] = g('x');
    d[8] = g('g');
    d[1] = g('o') - d[0] - d[2] - d[4];
    d[3] = g('h') - d[8];
    d[5] = g('f') - d[4];
    d[7] = g('s') - d[6];
    d[9] = g('i') - d[5] - d[6] - d[8];

    let out = "";
    for (let i = 0; i < 10; i++) out += String(i).repeat(d[i]);
    return out;
}

console.log(recoverDigits("niesevehrtfeev"));  // 357
