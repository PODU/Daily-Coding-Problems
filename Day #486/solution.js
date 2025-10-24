// Day 486: Celebrity problem.
// Two-pointer elimination: one candidate survives in O(n) knows() calls, then
// verify in O(n). Total O(n) time, O(1) space.

// mock relation matrix: KNOWS[a][b] === 1 means a knows b
let KNOWS = [];

function knows(a, b) {
  return KNOWS[a][b] === 1;
}

function findCelebrity(n) {
  let candidate = 0;
  for (let i = 1; i < n; i++) {
    if (knows(candidate, i)) candidate = i;
  }
  for (let i = 0; i < n; i++) {
    if (i === candidate) continue;
    if (knows(candidate, i) || !knows(i, candidate)) return -1;
  }
  return candidate;
}

// person 2 is the celebrity: knows nobody, everyone knows them
KNOWS = [
  [0, 1, 1, 0],
  [1, 0, 1, 1],
  [0, 0, 0, 0],
  [1, 1, 1, 0],
];
console.log(findCelebrity(4)); // 2
