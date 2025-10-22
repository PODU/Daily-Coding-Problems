// Count decodings (a=1..z=26) with bottom-up DP keeping only two running states.
// Time: O(n), Space: O(1).
"use strict";

function numDecodings(s) {
    if (s.length === 0 || s[0] === '0') return 0;
    let prev2 = 1; // ways up to i-2
    let prev1 = 1; // ways up to i-1
    for (let i = 1; i < s.length; i++) {
        let cur = 0;
        if (s[i] !== '0') cur += prev1;
        const two = (s.charCodeAt(i - 1) - 48) * 10 + (s.charCodeAt(i) - 48);
        if (two >= 10 && two <= 26) cur += prev2;
        prev2 = prev1;
        prev1 = cur;
    }
    return prev1;
}

function main() {
    const msg = "111";
    console.log(numDecodings(msg));
}

main();
