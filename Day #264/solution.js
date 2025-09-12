// Day 264: De Bruijn sequence B(k, n) over a character set.
// Approach: Fredricksen-Kessler-Maiorana algorithm — concatenate Lyndon words
// whose length divides n, generated via Duval-style recursion.
// Time O(k^n) (size of the output), Space O(n).

function deBruijn(alphabet, n) {
  const k = alphabet.length;
  const a = new Array(k * n).fill(0);
  const sequence = [];

  function db(t, p) {
    if (t > n) {
      if (n % p === 0) {
        for (let i = 1; i <= p; i++) sequence.push(alphabet[a[i]]);
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
  return sequence.join("");
}

function main() {
  // C = {0, 1}, k = 3
  console.log(deBruijn(["0", "1"], 3));
}

main();
