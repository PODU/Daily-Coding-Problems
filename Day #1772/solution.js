// Day 1772: Min jumps to reach N. Grow k until triangular sum >= |N| and (sum-|N|) even.
// Flipping any jump j changes parity of (sum-N) by 2j, so even surplus is reachable. O(sqrt(N)).
'use strict';

function minJumps(n) {
    const s = Math.abs(n);
    let sum = 0;
    let k = 0;
    while (sum < s || (sum - s) % 2 !== 0) {
        k++;
        sum += k;
    }
    return k;
}

console.log(minJumps(10));
