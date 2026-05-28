// Cryptarithmetic solver via backtracking over distinct letters. O(10!/(10-k)!) worst, pruned.

function solveCrypt(w1, w2, w3) {
  const letters = [];
  const seen = new Set();
  for (const w of [w1, w2, w3])
    for (const c of w)
      if (!seen.has(c)) { seen.add(c); letters.push(c); }
  const leading = new Set([w1[0], w2[0], w3[0]]);
  const assign = {};
  const used = new Array(10).fill(false);

  const val = (w) => {
    let v = 0;
    for (const c of w) v = v * 10 + assign[c];
    return v;
  };

  const bt = (idx) => {
    if (idx === letters.length) return val(w1) + val(w2) === val(w3);
    const c = letters[idx];
    for (let d = 0; d <= 9; d++) {
      if (used[d]) continue;
      if (d === 0 && leading.has(c)) continue;
      used[d] = true; assign[c] = d;
      if (bt(idx + 1)) return true;
      used[d] = false;
    }
    return false;
  };

  if (!bt(0)) return null;
  return { letters, assign };
}

const { letters, assign } = solveCrypt("SEND", "MORE", "MONEY");
console.log("{" + letters.map((c) => `'${c}': ${assign[c]}`).join(", ") + "}");
