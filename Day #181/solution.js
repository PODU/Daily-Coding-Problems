// Day 181: Minimum palindrome partitioning.
// DP: palindrome table + min-cut DP with reconstruction. Time O(n^2), Space O(n^2).
function minPalindromePartition(s) {
  const n = s.length;
  if (n === 0) return [];
  const pal = Array.from({ length: n }, () => new Array(n).fill(false));
  for (let i = 0; i < n; i++) pal[i][i] = true;
  for (let L = 2; L <= n; L++)
    for (let i = 0; i + L - 1 < n; i++) {
      const j = i + L - 1;
      if (s[i] === s[j] && (L === 2 || pal[i + 1][j - 1])) pal[i][j] = true;
    }
  const INF = Infinity;
  const cut = new Array(n + 1).fill(INF);
  const prev = new Array(n + 1).fill(-1);
  cut[0] = 0;
  for (let i = 1; i <= n; i++)
    for (let j = 0; j < i; j++)
      if (pal[j][i - 1] && cut[j] + 1 < cut[i]) {
        cut[i] = cut[j] + 1;
        prev[i] = j;
      }
  const res = [];
  for (let i = n; i > 0; i = prev[i]) res.push(s.slice(prev[i], i));
  res.reverse();
  return res;
}

function fmt(v) {
  return "[" + v.map((x) => '"' + x + '"').join(", ") + "]";
}

console.log(fmt(minPalindromePartition("racecarannakayak")));
console.log(fmt(minPalindromePartition("abc")));
