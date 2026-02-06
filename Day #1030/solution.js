// Day 1030: Quxes minimum remaining. Count colors; parity-based O(N) formula.
// If two counts are 0 -> n; else if all parities equal -> 2; else -> 1. Time O(N), Space O(1).
function minQuxes(q) {
  let r = 0, g = 0, b = 0;
  for (const c of q) {
    if (c === 'R') r++;
    else if (c === 'G') g++;
    else b++;
  }
  const n = r + g + b;
  const zeros = (r === 0) + (g === 0) + (b === 0);
  if (zeros >= 2) return n;
  if (r % 2 === g % 2 && g % 2 === b % 2) return 2;
  return 1;
}

console.log(minQuxes(['R', 'G', 'B', 'G', 'B']));
