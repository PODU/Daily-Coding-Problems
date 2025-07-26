// Edit Distance via DP. Time O(m*n), Space O(min(m,n)) using two rolling rows.
function editDistance(a, b) {
  if (a.length < b.length) { [a, b] = [b, a]; }
  const m = a.length, n = b.length;
  let prev = Array.from({ length: n + 1 }, (_, j) => j);
  for (let i = 1; i <= m; i++) {
    const cur = new Array(n + 1);
    cur[0] = i;
    for (let j = 1; j <= n; j++) {
      if (a[i - 1] === b[j - 1]) cur[j] = prev[j - 1];
      else cur[j] = 1 + Math.min(prev[j - 1], prev[j], cur[j - 1]);
    }
    prev = cur;
  }
  return prev[n];
}

console.log(editDistance("kitten", "sitting"));
