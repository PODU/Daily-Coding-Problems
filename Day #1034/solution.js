// Day 1034: Expected rounds until one of n fair coins remains.
// Model: expected max of n iid Geometric(1/2) lifetimes; E = sum_{m>=0}(1-(1-2^-m)^n). O(iter).
function expectedRounds(n) {
  let e = 0.0;
  let p = 1.0; // p = 2^-m
  for (let m = 0; m < 100000; m++) {
    const term = 1.0 - Math.pow(1.0 - p, n);
    if (term < 1e-12 && m > 0) break;
    e += term;
    p *= 0.5;
  }
  return e;
}

const n = 4;
console.log(`n=${n} -> expected rounds: ${expectedRounds(n).toFixed(4)}`);
