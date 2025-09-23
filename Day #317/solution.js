// Bitwise AND of all integers in [M, N] = common binary prefix.
// Shift both right until equal, then shift back. Time O(log N), Space O(1).

function rangeAnd(m, n) {
  let shift = 0;
  while (m !== n) {
    m >>= 1;
    n >>= 1;
    shift++;
  }
  return m << shift;
}

const M = 5, N = 7;
console.log(`Bitwise AND of [${M}, ${N}] = ${rangeAnd(M, N)}`);
