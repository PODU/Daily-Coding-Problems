// Quxes minimization: count R/G/B; two zero counts -> n; all parities equal -> 2; else 1.
// Time O(n), Space O(1).
function minQuxes(a) {
  let r = 0, g = 0, b = 0;
  for (const c of a) {
    if (c === 'R') r++;
    else if (c === 'G') g++;
    else b++;
  }
  const n = a.length;
  const zeros = (r === 0) + (g === 0) + (b === 0);
  if (zeros >= 2) return n;
  if ((r % 2) === (g % 2) && (g % 2) === (b % 2)) return 2;
  return 1;
}

const demo = ['R', 'G', 'B', 'G', 'B'];
console.log(minQuxes(demo));
