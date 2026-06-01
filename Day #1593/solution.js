// rand5 from rand7 via rejection sampling: draw rand7, accept if <=5 else retry.
// Expected O(1) calls (acceptance prob 5/7). Output uniform on 1..5.
'use strict';

// Deterministic seeded PRNG (mulberry32) so output is reproducible.
function mulberry32(seed) {
  let a = seed >>> 0;
  return function () {
    a |= 0;
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

const rand = mulberry32(12345);

// uniform 1..7 using language RNG
function rand7() {
  return Math.floor(rand() * 7) + 1;
}

// uniform 1..5 via rejection sampling
function rand5() {
  while (true) {
    const v = rand7();
    if (v <= 5) return v;
  }
}

const N = 100000;
const counts = new Array(6).fill(0);
for (let i = 0; i < N; i++) counts[rand5()]++;

console.log('Distribution over ' + N + ' samples:');
for (let v = 1; v <= 5; v++) console.log('  ' + v + ': ' + counts[v]);

const samples = [];
for (let i = 0; i < 10; i++) samples.push(rand5());
console.log('First 10 samples: ' + samples.join(' '));
