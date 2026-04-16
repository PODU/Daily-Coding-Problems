// Jump game: greedy track furthest reachable index. Time O(n), Space O(1).
function canReach(a) {
  let reach = 0;
  for (let i = 0; i < a.length; i++) {
    if (i > reach) return false;
    reach = Math.max(reach, i + a[i]);
  }
  return true;
}

console.log(canReach([2, 0, 1, 0]) ? "True" : "False");
console.log(canReach([1, 1, 0, 1]) ? "True" : "False");
