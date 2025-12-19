// Day 761: rand5() from rand7() via rejection sampling.
// Accept values 1..5, reject 6..7 and retry. Uniform; expected O(1) calls (7/5).
"use strict";

let s = 1n;
function rand7() {
    // BigInt: s * 1103515245 overflows the 2^53 float mantissa otherwise.
    s = (s * 1103515245n + 12345n) & 0x7fffffffn;
    return Number(s % 7n) + 1;   // uniform 1..7
}
function rand5() {
    let x;
    do { x = rand7(); } while (x > 5);
    return x;
}

const N = 100000;
const cnt = new Array(6).fill(0);
for (let i = 0; i < N; i++) cnt[rand5()]++;
console.log(`counts over ${N} samples:`);
for (let v = 1; v <= 5; v++) console.log(`${v}: ${cnt[v]}`);
