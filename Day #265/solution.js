// Day 265: Minimum bonuses. Two-pass scan (left-to-right then right-to-left),
// each worker gets max of the two passes. Time O(n), space O(n).

function bonuses(lines) {
  const n = lines.length;
  const b = new Array(n).fill(1);
  for (let i = 1; i < n; i++)
    if (lines[i] > lines[i - 1]) b[i] = b[i - 1] + 1;
  for (let i = n - 2; i >= 0; i--)
    if (lines[i] > lines[i + 1]) b[i] = Math.max(b[i], b[i + 1] + 1);
  return b;
}

const lines = [10, 40, 200, 1000, 60, 30];
console.log("[" + bonuses(lines).join(", ") + "]");
