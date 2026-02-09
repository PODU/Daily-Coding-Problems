// Apply permutation where result[P[i]] = array[i] (scatter). In-place cycle-following via swaps:
// O(n) time, O(1) extra space. Also a simple O(n)-space scatter is provided.

function applyInPlace(a, P) {
  P = P.slice(); // local copy so caller's permutation is untouched
  for (let i = 0; i < a.length; i++) {
    while (P[i] !== i) {
      const j = P[i];
      [a[i], a[j]] = [a[j], a[i]];
      [P[i], P[j]] = [P[j], P[i]];
    }
  }
  return a;
}

function applySimple(a, P) {
  const res = new Array(a.length);
  for (let i = 0; i < a.length; i++) res[P[i]] = a[i];
  return res;
}

const a = ["a", "b", "c"];
const P = [2, 1, 0]; // result[P[i]] = a[i]
applyInPlace(a, P);
console.log("[" + a.join(", ") + "]");
