// Cryptarithmetic solver via backtracking over letter->digit assignments.
// Time O(10!/(10-L)!) worst with pruning, Space O(L). L = #distinct letters.
function solve(a, b, c) {
  const words = [a, b, c];
  const letters = [], seen = new Set();
  for (const w of words)
    for (const ch of w)
      if (!seen.has(ch)) { seen.add(ch); letters.push(ch); }
  const leading = new Set(words.map((w) => w[0]));
  const assign = {}, used = new Array(10).fill(false);

  const val = (w) => {
    let v = 0;
    for (const ch of w) v = v * 10 + assign[ch];
    return v;
  };

  const bt = (i) => {
    if (i === letters.length) return val(a) + val(b) === val(c);
    const ch = letters[i];
    for (let d = 0; d < 10; d++) {
      if (used[d] || (d === 0 && leading.has(ch))) continue;
      used[d] = true; assign[ch] = d;
      if (bt(i + 1)) return true;
      used[d] = false;
    }
    return false;
  };

  if (bt(0)) {
    const obj = {};
    for (const ch of letters) obj[ch] = assign[ch];
    return obj;
  }
  return null;
}

const res = solve("SEND", "MORE", "MONEY");
const parts = Object.entries(res).map(([k, v]) => `'${k}': ${v}`);
console.log(`{${parts.join(", ")}}`);
