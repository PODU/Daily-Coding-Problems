// Day 933: Reconstruct a permutation of [0..N] consistent with +/- signs.
// Two-pointer: '+' takes the current low, '-' takes the current high. O(n) time, O(n) space.
// Many arrangements are valid; README shows [1,2,3,0,4], this prints another valid one.
function reconstruct(signs) {
  // signs[0] is null; signs[i] in {'+','-'} for i>=1.
  const n = signs.length;
  let lo = 0, hi = n - 1;
  const res = [];
  for (let i = 1; i < n; i++) {
    if (signs[i] === '+') res.push(lo++);
    else res.push(hi--);
  }
  res.push(lo); // lo === hi
  return res;
}

const signs = [null, '+', '+', '-', '+'];
console.log(reconstruct(signs)); // e.g. [ 0, 1, 4, 2, 3 ] (a valid reconstruction)
