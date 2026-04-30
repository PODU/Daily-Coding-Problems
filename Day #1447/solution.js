// Day 1447: Does a secret code (6 distinct digits) exist consistent with all
// (guess, score) pairs? Brute force all 6-permutations of 0-9. Time O(P*G*6).
function scoreMatch(code, guess) {
  let s = 0;
  for (let i = 0; i < 6; i++) if (code[i] === guess.charCodeAt(i) - 48) s++;
  return s;
}

function consistent(guesses) {
  const items = guesses.map(([g, s]) => [String(g).padStart(6, "0"), s]);
  const code = new Array(6);
  const used = new Array(10).fill(false);

  function dfs(pos) {
    if (pos === 6) {
      return items.every(([g, sc]) => scoreMatch(code, g) === sc);
    }
    for (let d = 0; d < 10; d++) {
      if (used[d]) continue;
      if (pos === 0 && d === 0) continue; // no leading zero
      used[d] = true; code[pos] = d;
      if (dfs(pos + 1)) { used[d] = false; return true; }
      used[d] = false;
    }
    return false;
  }
  return dfs(0);
}

const e1 = [[175286, 2], [293416, 3], [654321, 0]];
const e2 = [[123456, 4], [345678, 4], [567890, 4]];
console.log(consistent(e1) ? "True" : "False"); // True
console.log(consistent(e2) ? "True" : "False"); // False
