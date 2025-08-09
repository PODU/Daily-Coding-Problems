// Day 90: Uniform random in [0,n) excluding l. Pick r in [0, n-k), then offset past
// sorted exclusions so every valid value is equally likely. Time O(k log k), Space O(k).
function randomExcluding(n, l) {
  const ex = [...new Set(l.filter((v) => v >= 0 && v < n))].sort((a, b) => a - b);
  const m = n - ex.length;
  if (m <= 0) throw new Error("no valid number");
  let r = Math.floor(Math.random() * m);
  for (const e of ex) {
    if (e <= r) r++;
    else break;
  }
  return r;
}

console.log(randomExcluding(5, [1, 3])); // sample from {0, 2, 4}
