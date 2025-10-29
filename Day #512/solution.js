// Jump Game: greedily track the furthest reachable index. O(n) time, O(1) space.
function canJump(a) {
  let reach = 0;
  for (let i = 0; i < a.length; i++) {
    if (i > reach) return false;
    reach = Math.max(reach, i + a[i]);
  }
  return true;
}

console.log(canJump([1, 3, 1, 2, 0, 1]));
console.log(canJump([1, 2, 1, 0, 0]));
