// Day 1097: Smallest string by moving one of first k letters to the end.
// k==1 -> min rotation; k>=2 -> any permutation reachable -> sorted string.
// Time: O(N^2) for k==1, O(N log N) for k>=2. Space: O(N).
function smallest(s, k) {
  if (k >= 2) return s.split("").sort().join("");
  let best = s, cur = s;
  for (let i = 0; i < s.length; i++) {
    cur = cur.slice(1) + cur[0];
    if (cur < best) best = cur;
  }
  return best;
}

console.log(smallest("daily", 1)); // ailyd
