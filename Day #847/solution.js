// Day 847: jump game - can we reach the last index? Greedy furthest-reach. O(n) time, O(1) space.
function canReach(a) {
  let reach = 0;
  for (let i = 0; i < a.length; i++) {
    if (i > reach) return false;
    reach = Math.max(reach, i + a[i]);
  }
  return true;
}

console.log(canReach([2, 0, 1, 0])); // true
console.log(canReach([1, 1, 0, 1])); // false
