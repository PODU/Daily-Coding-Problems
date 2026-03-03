// Day 1146: Dominoes - two-pass force accumulation.
// L->R pass adds rightward force, R->L pass adds leftward; sign decides. O(n) time, O(n) space.
function pushDominoes(s) {
  const n = s.length;
  const forces = new Array(n).fill(0);
  let force = 0;
  for (let i = 0; i < n; i++) {
    if (s[i] === "R") force = n;
    else if (s[i] === "L") force = 0;
    else force = Math.max(force - 1, 0);
    forces[i] += force;
  }
  force = 0;
  for (let i = n - 1; i >= 0; i--) {
    if (s[i] === "L") force = n;
    else if (s[i] === "R") force = 0;
    else force = Math.max(force - 1, 0);
    forces[i] -= force;
  }
  return forces.map((f) => (f > 0 ? "R" : f < 0 ? "L" : ".")).join("");
}

console.log(pushDominoes(".L.R....L")); // LL.RRRLLL
console.log(pushDominoes("..R...L.L")); // ..RR.LLLL
