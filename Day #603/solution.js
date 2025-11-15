// Day 603: Final resting state of pushed dominoes.
// Approach: two force passes (R rightward, L leftward), compare. Time O(n), Space O(n).
'use strict';

function pushDominoes(s) {
    const n = s.length;
    const fR = new Array(n).fill(0), fL = new Array(n).fill(0);
    let f = 0;
    for (let i = 0; i < n; i++) {
        if (s[i] === 'R') f = n;
        else if (s[i] === 'L') f = 0;
        else f = Math.max(f - 1, 0);
        fR[i] = f;
    }
    f = 0;
    for (let i = n - 1; i >= 0; i--) {
        if (s[i] === 'L') f = n;
        else if (s[i] === 'R') f = 0;
        else f = Math.max(f - 1, 0);
        fL[i] = f;
    }
    let res = '';
    for (let i = 0; i < n; i++) {
        if (fR[i] > fL[i]) res += 'R';
        else if (fR[i] < fL[i]) res += 'L';
        else res += '.';
    }
    return res;
}

function main() {
    console.log(pushDominoes(".L.R....L")); // LL.RRRLLL
    console.log(pushDominoes("..R...L.L")); // ..RR.LLLL
}

main();
