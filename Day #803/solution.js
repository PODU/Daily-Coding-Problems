// Day 803: Does a secret (6 distinct digits) exist matching all guess scores?
// Brute force all 6-digit distinct-digit codes, verify every guess's score.
// Time O(10^6 * G), Space O(G).

function digits(x) {
  const d = new Array(6);
  for (let i = 5; i >= 0; i--) { d[i] = x % 10; x = Math.floor(x / 10); }
  return d;
}

function distinct(d) {
  let seen = 0;
  for (const v of d) { if (seen & (1 << v)) return false; seen |= 1 << v; }
  return true;
}

function score(code, guess) {
  let s = 0;
  for (let i = 0; i < 6; i++) if (code[i] === guess[i]) s++;
  return s;
}

function feasible(guesses) {
  const gs = Object.entries(guesses).map(([g, sc]) => [digits(+g), sc]);
  for (let code = 0; code <= 999999; code++) {
    const d = digits(code);
    if (!distinct(d)) continue;
    if (gs.every(([gd, sc]) => score(d, gd) === sc)) return true;
  }
  return false;
}

console.log(feasible({ 175286: 2, 293416: 3, 654321: 0 })); // true
console.log(feasible({ 123456: 4, 345678: 4, 567890: 4 })); // false
