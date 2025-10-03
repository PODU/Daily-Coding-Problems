// Day 361: Mastermind feasibility.
// Brute-force all 6-position codes with distinct digits; accept if some code
// matches every guess's score. Time O(P*G*6), P=151200, Space O(1).
'use strict';

function scoreOf(code, guess) {
  let s = 0;
  for (let i = 0; i < 6; i++) if (code[i] === guess[i]) s++;
  return s;
}

function feasible(guesses) {
  const codes = Object.keys(guesses).map((k) => k.padStart(6, '0'));
  const scores = Object.values(guesses);
  const code = new Array(6);

  function rec(pos, used) {
    if (pos === 6) {
      const c = code.join('');
      for (let i = 0; i < codes.length; i++)
        if (scoreOf(c, codes[i]) !== scores[i]) return false;
      return true;
    }
    for (let d = 0; d < 10; d++)
      if (!(used & (1 << d))) {
        code[pos] = String(d);
        if (rec(pos + 1, used | (1 << d))) return true;
      }
    return false;
  }
  return rec(0, 0);
}

console.log(feasible({ 175286: 2, 293416: 3, 654321: 0 }) ? 'True' : 'False');
console.log(feasible({ 123456: 4, 345678: 4, 567890: 4 }) ? 'True' : 'False');
