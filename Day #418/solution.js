// Day 418: Two-pass greedy. Each gets >= 1; more than any lower neighbor. Like candy distribution.
// Time O(n), Space O(n).
function bonuses(lines) {
  const n = lines.length;
  const res = new Array(n).fill(1);
  for (let i = 1; i < n; i++)
    if (lines[i] > lines[i - 1]) res[i] = res[i - 1] + 1;
  for (let i = n - 2; i >= 0; i--)
    if (lines[i] > lines[i + 1]) res[i] = Math.max(res[i], res[i + 1] + 1);
  return res;
}

const res = bonuses([10, 40, 200, 1000, 60, 30]);
console.log("[" + res.join(", ") + "]");
