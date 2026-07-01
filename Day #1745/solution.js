// Celebrity finder: two-phase candidate elimination then verify. O(N) knows() calls, O(1) space.

const knowsMat = [
  [0, 1, 1, 0],
  [0, 0, 1, 0],
  [0, 0, 0, 0],
  [0, 1, 1, 0],
];

const knows = (a, b) => knowsMat[a][b] === 1;

function findCelebrity(n) {
  let cand = 0;
  for (let i = 1; i < n; i++) if (knows(cand, i)) cand = i;
  for (let i = 0; i < n; i++) {
    if (i === cand) continue;
    if (knows(cand, i) || !knows(i, cand)) return -1;
  }
  return cand;
}

function main() {
  console.log(findCelebrity(4));
}

main();
