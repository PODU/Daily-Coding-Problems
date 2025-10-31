// k==1: smallest rotation (try all n rotations). k>=2: sorted string (any pair
// of front letters can be reordered). Time O(n^2) for k==1, O(n log n) k>=2.
function smallest(s, k) {
  if (k >= 2) return s.split("").sort().join("");
  let best = s;
  for (let i = 1; i < s.length; i++) {
    const rot = s.slice(i) + s.slice(0, i);
    if (rot < best) best = rot;
  }
  return best;
}

console.log(smallest("daily", 1));
