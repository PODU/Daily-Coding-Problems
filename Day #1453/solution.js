// Day 1453: Apply a permutation P (P[i] = destination of element i) to an array
// in place by following cycles. Time O(n), Space O(1) extra (P is consumed).
function applyPermutation(arr, P) {
  for (let i = 0; i < arr.length; i++) {
    while (P[i] !== i) {
      const pi = P[i];
      [arr[i], arr[pi]] = [arr[pi], arr[i]];
      [P[i], P[pi]] = [P[pi], P[i]];
    }
  }
}

const arr = ["a", "b", "c"];
const P = [2, 1, 0];
applyPermutation(arr, P);
console.log("[" + arr.map((x) => `"${x}"`).join(", ") + "]"); // ["c", "b", "a"]
