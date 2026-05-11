// Jump game: greedy, track furthest reachable index.
// Time O(n), Space O(1). Prints "True"/"False" (capitalized) per spec.
function canJump(a) {
  let reach = 0;
  for (let i = 0; i < a.length; i++) {
    if (i > reach) return false;
    reach = Math.max(reach, i + a[i]);
  }
  return true;
}

console.log(canJump([2, 0, 1, 0]) ? "True" : "False");
