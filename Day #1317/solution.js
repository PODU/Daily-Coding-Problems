// Reconstruct a permutation of [0..N] consistent with up/down signs.
// Two-pointer (DI-match): '+' takes next low, '-' takes next high. Time O(N).
// Any consistent array is valid; README shows one such answer.
'use strict';

// signs[0] is null (None); signs[i] in {'+','-'} for i>=1.
function reconstruct(signs) {
  const n = signs.length, N = n - 1;
  const res = [];
  let lo = 0, hi = N;
  for (let i = 1; i < n; i++) {
    res.push(signs[i] === '+' ? lo++ : hi--);
  }
  res.push(lo); // lo == hi for the final element
  return res;
}

function consistent(signs, a) {
  for (let i = 1; i < signs.length; i++) {
    if (signs[i] === '+' && !(a[i] > a[i - 1])) return false;
    if (signs[i] === '-' && !(a[i] < a[i - 1])) return false;
  }
  return true;
}

const signs = [null, '+', '+', '-', '+'];
const a = reconstruct(signs);
console.log(`[${a.join(', ')}]  consistent=${consistent(signs, a)}`);
// -> [0, 1, 4, 2, 3]  (README's [1, 2, 3, 0, 4] is another valid answer)
