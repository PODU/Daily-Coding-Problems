// rand7 from rand5 via rejection sampling: combine two rand5 into uniform
// 1..25, accept 1..21, map to 1..7. Expected O(1) calls, O(1) space.
'use strict';

let seed = 42;
function rnd() { seed = (seed * 1103515245 + 12345) & 0x7fffffff; return seed; }
function rand5() { return (rnd() % 5) + 1; } // uniform 1..5

function rand7() {
  for (;;) {
    const v = (rand5() - 1) * 5 + (rand5() - 1); // uniform 0..24
    if (v < 21) return (v % 7) + 1;              // accept 0..20 -> 1..7
  }
}

const counts = new Array(8).fill(0);
for (let i = 0; i < 70000; i++) counts[rand7()]++;
for (let i = 1; i <= 7; i++) console.log(`${i}: ${counts[i]}`);
