// Celebrity problem: 2-pass. Pass 1 eliminate to one candidate; pass 2 verify.
// Time O(n), Space O(1).
const M = [
  [0, 1, 1, 0], // person0 knows {1,2}
  [0, 0, 1, 0], // person1 knows {2}
  [0, 0, 0, 0], // person2 knows {}
  [1, 1, 1, 0], // person3 knows {0,1,2}
];

function knows(a, b) { return M[a][b] === 1; }

function findCelebrity(n) {
  let cand = 0;
  for (let i = 1; i < n; i++)
    if (knows(cand, i)) cand = i;
  for (let i = 0; i < n; i++) {
    if (i === cand) continue;
    if (knows(cand, i) || !knows(i, cand)) return -1;
  }
  return cand;
}

console.log(findCelebrity(4));
