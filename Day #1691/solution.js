// Next bigger integer with the same number of set bits (Gosper's hack). O(1).
'use strict';

function nextSamePopcount(n){
    const c = n & (-n);                          // lowest set bit
    const r = n + c;                             // ripple the carry
    return r | Math.floor(((n ^ r) >> 2) / c);   // restore trailing ones
}

console.log(nextSamePopcount(6)); // 6 (0110) -> 9 (1001)
