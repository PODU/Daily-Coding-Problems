// Celebrity problem: O(N) elimination to one candidate, then O(N) verify.
const M = [
  [0, 1, 1, 0],
  [0, 0, 1, 0],
  [0, 0, 0, 0],
  [0, 1, 1, 0],
];
const knows = (a, b) => M[a][b] === 1;

function findCelebrity(n) {
  let cand = 0;
  for (let i = 1; i < n; i++) if (knows(cand, i)) cand = i;
  for (let i = 0; i < n; i++)
    if (i !== cand && (knows(cand, i) || !knows(i, cand))) return -1;
  return cand;
}

console.log(findCelebrity(4));
