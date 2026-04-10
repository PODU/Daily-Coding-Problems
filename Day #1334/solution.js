// Day 1334: Levenshtein edit distance between two strings.
// Classic DP with rolling row. O(n*m) time, O(min(n,m)) space.

function editDistance(a, b) {
  const n = a.length, m = b.length;
  let prev = Array.from({ length: m + 1 }, (_, j) => j);
  for (let i = 1; i <= n; i++) {
    const cur = new Array(m + 1);
    cur[0] = i;
    for (let j = 1; j <= m; j++) {
      if (a[i - 1] === b[j - 1]) cur[j] = prev[j - 1];
      else cur[j] = 1 + Math.min(prev[j - 1], prev[j], cur[j - 1]);
    }
    prev = cur;
  }
  return prev[m];
}

console.log(editDistance("kitten", "sitting")); // 3
