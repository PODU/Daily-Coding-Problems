// Celebrity problem: one candidate via elimination, then verify.
// Two-pointer elimination + verification. Time: O(N) knows calls, Space: O(1).

// Demo knows matrix: N=4, person 2 is the celebrity.
const M = [
  [0, 1, 1, 0], // 0 knows 2
  [0, 0, 1, 0], // 1 knows 2
  [0, 0, 0, 0], // 2 (celebrity) knows no one
  [0, 1, 1, 0], // 3 knows 2
];

function knows(a, b) {
  return M[a][b] === 1;
}

function findCelebrity(n) {
  let cand = 0;
  for (let i = 1; i < n; i++) if (knows(cand, i)) cand = i;
  for (let i = 0; i < n; i++) {
    if (i === cand) continue;
    if (knows(cand, i) || !knows(i, cand)) return -1;
  }
  return cand;
}

console.log(findCelebrity(M.length));
