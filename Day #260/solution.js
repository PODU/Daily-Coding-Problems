// Day 260: Reconstruct a permutation of [0..N] consistent with +/- sign array.
// Grow a list: on '+' append max+1, on '-' append min-1; shift by -min into [0..N].
// O(n) time, O(n) space.

function reconstruct(signs) { // signs[0] is null (None)
  const res = [0];
  let curMax = 0, curMin = 0;
  for (let i = 1; i < signs.length; i++) {
    if (signs[i] > 0) res.push(++curMax);
    else res.push(--curMin);
  }
  const off = -curMin;
  return res.map((x) => x + off);
}

const signs = [null, 1, 1, -1, 1]; // [None, +, +, -, +]
console.log("[" + reconstruct(signs).join(", ") + "]");
