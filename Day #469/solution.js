// Mastermind: brute force all 6-permutations of digits 0-9 (10P6=151200),
// keep one consistent with every guess score. Time: O(10P6 * G), Space: O(1).
function score(secret, guess) {
  let s = 0;
  for (let i = 0; i < 6; i++) if (secret[i] === guess[i]) s++;
  return s;
}

function consistent(guesses) {
  const items = Object.entries(guesses).map(([g, s]) => [g.padStart(6, "0"), s]);
  const used = new Array(10).fill(false);
  const secret = new Array(6);

  function search(pos) {
    if (pos === 6) {
      const str = secret.join("");
      return items.every(([g, s]) => score(str, g) === s);
    }
    for (let d = 0; d < 10; d++) {
      if (used[d]) continue;
      used[d] = true;
      secret[pos] = String(d);
      if (search(pos + 1)) return true;
      used[d] = false;
    }
    return false;
  }
  return search(0);
}

const ex1 = { 175286: 2, 293416: 3, 654321: 0 };
const ex2 = { 123456: 4, 345678: 4, 567890: 4 };
console.log(consistent(ex1) ? "True" : "False");
console.log(consistent(ex2) ? "True" : "False");
