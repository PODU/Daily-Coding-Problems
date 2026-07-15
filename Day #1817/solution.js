// De Bruijn sequence B(k_alphabet, n) via the FKM/Lyndon-word recursion.
// Concatenating Lyndon words whose length divides n. Time: O(k^n). Space: O(k^n).
function deBruijn(C, n) {
  const k = C.length;
  const a = new Array(k * n).fill(0);
  const seq = [];
  function db(t, p) {
    if (t > n) {
      if (n % p === 0) for (let j = 1; j <= p; j++) seq.push(a[j]);
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
  return seq.map((i) => C[i]).join("");
}

console.log(deBruijn(["0", "1"], 3)); // 00010111
