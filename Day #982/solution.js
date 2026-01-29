// Strobogrammatic numbers of N digits: build recursively from outside in using rotatable
// pairs {0-0,1-1,8-8,6-9,9-6}; skip leading 0 for N>1. Time O(5^(N/2)), Space O(N).
const PAIRS = [['0', '0'], ['1', '1'], ['8', '8'], ['6', '9'], ['9', '6']];

function build(n, total) {
  if (n === 0) return [""];
  if (n === 1) return ["0", "1", "8"];
  const inner = build(n - 2, total);
  const res = [];
  for (const s of inner)
    for (const [a, b] of PAIRS) {
      if (a === '0' && n === total) continue; // no leading zero
      res.push(a + s + b);
    }
  return res;
}

const strobogrammatic = n => build(n, n);

console.log("N=2:", strobogrammatic(2));
console.log("N=1:", strobogrammatic(1));
