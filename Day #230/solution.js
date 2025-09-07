// Egg drop: find min trials t such that maxFloors(t, eggs) >= k, where
// f(t,e) = f(t-1,e-1) + f(t-1,e) + 1 (floors distinguishable). Time: O(eggs * answer), Space: O(eggs).
function eggDrop(eggs, k) {
  const f = new Array(eggs + 1).fill(0);
  let t = 0;
  while (f[eggs] < k) {
    t++;
    for (let e = eggs; e >= 1; e--) f[e] = f[e] + f[e - 1] + 1;
  }
  return t;
}

console.log(eggDrop(1, 5)); // 5
