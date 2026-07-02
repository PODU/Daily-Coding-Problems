// Day 1753: Egyptian fraction (sum of distinct unit fractions).
// Greedy Fibonacci-Sylvester: repeatedly subtract 1/ceil(b/a). Time O(terms), O(1) space.
'use strict';

function gcd(x, y) {
  while (y !== 0n) { const t = x % y; x = y; y = t; }
  return x;
}

function egyptian(a, b) {
  a = BigInt(a);
  b = BigInt(b);
  const terms = [];
  while (a !== 0n) {
    const c = (b + a - 1n) / a; // ceil(b / a)
    terms.push("1 / " + c.toString());
    a = a * c - b;
    b = b * c;
    const g = gcd(a < 0n ? -a : a, b);
    if (g > 1n) { a /= g; b /= g; }
  }
  return terms.join(" + ");
}

// README example: 4 / 13 -> 1 / 4 + 1 / 18 + 1 / 468
console.log(egyptian(4, 13));
