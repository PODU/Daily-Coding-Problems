// De Bruijn B(k,n) via FKM (Lyndon word) algorithm -> lexicographically smallest. O(k^n).
function deBruijn(k, n) {
  const a = new Array(k * n + 1).fill(0);
  const seq = [];

  function db(t, p) {
    if (t > n) {
      if (n % p === 0) {
        for (let i = 1; i <= p; i++) seq.push(a[i]);
      }
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

// C = {0,1} -> alphabet size 2, substring length 3
console.log(deBruijn(2, 3));
