// Day 347: Lexicographically smallest string by moving one of first k letters to the end.
// k==1 -> best rotation; k>=2 -> any permutation reachable, so sorted. Time O(N^2)/O(N log N).
function smallest(s, k) {
  if (k === 1) {
    let best = s;
    for (let i = 1; i < s.length; i++) {
      const rot = s.slice(i) + s.slice(0, i);
      if (rot < best) best = rot;
    }
    return best;
  }
  return s.split('').sort().join('');
}

function main() {
  console.log(smallest('daily', 1));
}

main();
