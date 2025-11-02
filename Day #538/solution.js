// De Bruijn sequence via FKM (Lyndon-word/necklace) algorithm: emit Lyndon words whose
// length divides n, in order, giving lexicographically smallest sequence. Time O(k^n).
'use strict';

function deBruijn(k, n) {
  const a = new Array(k * n + 1).fill(0);
  const seq = [];
  function db(t, p) {
    if (t > n) {
      if (n % p === 0)
        for (let j = 1; j <= p; j++) seq.push(a[j]);
    } else {
      a[t] = a[t - p];
      db(t + 1, p);
      for (let j = a[t - p] + 1; j < k; j++) {
        a[t] = j;
        db(t + 1, t);
      }
    }
  }
  db(1, 1);
  return seq.join("");
}

console.log(deBruijn(2, 3)); // 00010111
