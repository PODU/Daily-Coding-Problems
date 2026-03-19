// Min Quxes remaining. One color -> N; all counts same parity -> 2; else 1.
// Time O(n), Space O(1).
function minQuxes(q) {
  if (q.length === 0) return 0;
  let r = 0, g = 0, b = 0;
  for (const c of q) { if (c === 'R') r++; else if (c === 'G') g++; else b++; }
  const distinct = (r > 0) + (g > 0) + (b > 0);
  if (distinct === 1) return q.length;
  if (r % 2 === g % 2 && g % 2 === b % 2) return 2;
  return 1;
}

console.log(minQuxes(['R', 'G', 'B', 'G', 'B'].join('')));
