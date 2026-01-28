// Cryptarithmetic 3-word solver (word1 + word2 = word3) via backtracking.
// Time: O(10!/(10-k)!) for k unique letters; Space: O(k).
function solve(A, B, C) {
  const letters = [], seen = new Set();
  for (const w of [A, B, C])
    for (const ch of w)
      if (!seen.has(ch)) { seen.add(ch); letters.push(ch); }
  const leading = new Set([A[0], B[0], C[0]]);
  const assign = {}, used = new Array(10).fill(false);
  const val = w => { let v = 0; for (const ch of w) v = v * 10 + assign[ch]; return v; };
  function dfs(i) {
    if (i === letters.length) return val(A) + val(B) === val(C);
    const ch = letters[i];
    for (let d = 0; d < 10; d++) {
      if (used[d]) continue;
      if (d === 0 && leading.has(ch)) continue;
      used[d] = true; assign[ch] = d;
      if (dfs(i + 1)) return true;
      used[d] = false;
    }
    return false;
  }
  dfs(0);
  return assign;
}

const m = solve("SEND", "MORE", "MONEY");
const order = "SENDMORY";
console.log("{" + [...order].map(c => `'${c}': ${m[c]}`).join(", ") + "}");
