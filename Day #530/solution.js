// Levenshtein edit distance via DP with rolling array.
// Time O(m*n), Space O(min(m,n)).
function editDistance(a, b) {
  if (a.length < b.length) [a, b] = [b, a];
  const n = b.length;
  let prev = Array.from({ length: n + 1 }, (_, j) => j);
  for (let i = 1; i <= a.length; i++) {
    const cur = new Array(n + 1);
    cur[0] = i;
    for (let j = 1; j <= n; j++) {
      const cost = a[i - 1] === b[j - 1] ? 0 : 1;
      cur[j] = Math.min(prev[j] + 1, cur[j - 1] + 1, prev[j - 1] + cost);
    }
    prev = cur;
  }
  return prev[n];
}

console.log(editDistance("kitten", "sitting"));
