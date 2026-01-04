// Day 850: De Bruijn sequence via the FKM (Lyndon-word) algorithm.
// Lexicographically smallest cyclic sequence containing every length-n string once. O(k^n).
function deBruijn(k, n, alphabet) {
  const a = new Array(k * n).fill(0);
  const seq = [];
  function db(t, p) {
    if (t > n) {
      if (n % p === 0) for (let j = 1; j <= p; j++) seq.push(a[j]);
    } else {
      a[t] = a[t - p];
      db(t + 1, p);
      for (let j = a[t - p] + 1; j < k; j++) { a[t] = j; db(t + 1, t); }
    }
  }
  db(1, 1);
  return seq.map((i) => alphabet[i]).join("");
}

// C = {0,1}, length 3 => alphabet size 2, n = 3
console.log(deBruijn(2, 3, "01")); // 00010111
