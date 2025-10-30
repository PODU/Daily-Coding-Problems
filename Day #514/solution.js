// Longest consecutive sequence: hash set, extend only from sequence starts. O(n) time/space.
function longestConsecutive(a) {
  const s = new Set(a);
  let best = 0;
  for (const x of s) {
    if (s.has(x - 1)) continue;
    let len = 1, cur = x;
    while (s.has(cur + 1)) { cur++; len++; }
    best = Math.max(best, len);
  }
  return best;
}

console.log(longestConsecutive([100, 4, 200, 1, 3, 2]));
