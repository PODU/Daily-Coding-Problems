// Day 604: Soundex phonetic encoding (letter + 3 digits).
// Approach: keep first letter, code consonants, drop repeats/vowels, pad. Time O(L), Space O(L).
'use strict';

function code(c) {
    if ("BFPV".includes(c)) return '1';
    if ("CGJKQSXZ".includes(c)) return '2';
    if ("DT".includes(c)) return '3';
    if (c === 'L') return '4';
    if ("MN".includes(c)) return '5';
    if (c === 'R') return '6';
    if ("HW".includes(c)) return 'S'; // transparent
    return '0';                       // vowels
}

function soundex(name) {
    const up = name.toUpperCase().split('').filter(c => c >= 'A' && c <= 'Z');
    if (up.length === 0) return "0000";
    let res = up[0];
    let prev = code(up[0]);
    for (let i = 1; i < up.length && res.length < 4; i++) {
        const c = code(up[i]);
        if (c >= '1' && c <= '6') {
            if (c !== prev) res += c;
            prev = c;
        } else if (c === '0') {
            prev = '0';
        }
    }
    while (res.length < 4) res += '0';
    return res.slice(0, 4);
}

function main() {
    console.log(soundex("Jackson")); // J250
    console.log(soundex("Jaxen"));   // J250
}

main();
