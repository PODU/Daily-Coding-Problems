// Day 643: Longest common subsequence of three strings.
// Approach: 3D DP over prefix lengths of a, b, c.
// Time: O(|a|*|b|*|c|), Space: O(|b|*|c|) (two rolling layers).
function lcs3(a, b, c) {
  const B = b.length, C = c.length;
  let prev = Array.from({ length: B + 1 }, () => new Array(C + 1).fill(0));
  for (let i = 1; i <= a.length; i++) {
    const cur = Array.from({ length: B + 1 }, () => new Array(C + 1).fill(0));
    for (let j = 1; j <= B; j++)
      for (let k = 1; k <= C; k++) {
        if (a[i - 1] === b[j - 1] && b[j - 1] === c[k - 1])
          cur[j][k] = prev[j - 1][k - 1] + 1;
        else
          cur[j][k] = Math.max(prev[j][k], cur[j - 1][k], cur[j][k - 1]);
      }
    prev = cur;
  }
  return prev[B][C];
}

console.log(lcs3("epidemiologist", "refrigeration",
  "supercalifragilisticexpialodocious")); // 5
