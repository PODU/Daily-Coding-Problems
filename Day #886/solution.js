// Edit distance (Levenshtein) via DP. Time O(n*m), Space O(min(n,m)).
function editDistance(a, b) {
  if (a.length > b.length) [a, b] = [b, a];
  const n = a.length, m = b.length;
  let prev = Array.from({ length: n + 1 }, (_, i) => i);
  for (let j = 1; j <= m; j++) {
    const cur = new Array(n + 1);
    cur[0] = j;
    for (let i = 1; i <= n; i++) {
      const cost = a[i - 1] === b[j - 1] ? 0 : 1;
      cur[i] = Math.min(prev[i] + 1, cur[i - 1] + 1, prev[i - 1] + cost);
    }
    prev = cur;
  }
  return prev[n];
}

console.log(editDistance("kitten", "sitting"));
