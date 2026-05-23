// Candy problem: init bonuses to 1, left-to-right then right-to-left passes enforce strict ordering.
// Time O(n), Space O(n).

function bonuses(a) {
  const n = a.length;
  const b = new Array(n).fill(1);
  for (let i = 1; i < n; i++)
    if (a[i] > a[i - 1]) b[i] = b[i - 1] + 1;
  for (let i = n - 2; i >= 0; i--)
    if (a[i] > a[i + 1]) b[i] = Math.max(b[i], b[i + 1] + 1);
  return b;
}

const a = [10, 40, 200, 1000, 60, 30];
console.log("[" + bonuses(a).join(", ") + "]");
