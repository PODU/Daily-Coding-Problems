// Decode ways: DP rolling two states. Add prev1 if current digit valid, prev2 if last two in 10..26.
// Time: O(n), Space: O(1).
'use strict';

function numDecodings(s) {
    if (s.length === 0 || s[0] === '0') return 0;
    let prev2 = 1, prev1 = 1;
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
    console.log(numDecodings("111"));
}

main();
