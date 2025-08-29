// Day 192: Jump game -- can we reach the end (each value caps the jump length).
// Greedy farthest-reach. Time O(n), Space O(1).
function canReachEnd(a) {
  let reach = 0;
  for (let i = 0; i < a.length; i++) {
    if (i > reach) return false;
    reach = Math.max(reach, i + a[i]);
  }
  return true;
}

console.log(canReachEnd([1, 3, 1, 2, 0, 1]));
console.log(canReachEnd([1, 2, 1, 0, 0]));
