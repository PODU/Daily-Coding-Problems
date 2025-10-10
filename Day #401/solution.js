// Apply permutation P: result[P[i]] = array[i]. O(n) time, O(n) space.
// (In-place alternative: follow cycles swapping elements.)
function applyPermutation(array, P) {
  const result = new Array(array.length);
  for (let i = 0; i < array.length; i++) result[P[i]] = array[i];
  return result;
}

function main() {
  const array = ["a", "b", "c"];
  const P = [2, 1, 0];
  const res = applyPermutation(array, P);
  console.log("[" + res.map((x) => `"${x}"`).join(", ") + "]");
}

main();
