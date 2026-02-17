// B is a rotation of A iff lengths match and B is a substring of A+A. Time O(n), Space O(n).
function isRotation(a, b) {
  return a.length === b.length && (a + a).includes(b);
}

console.log(isRotation("abcde", "cdeab") ? "true" : "false");
console.log(isRotation("abc", "acb") ? "true" : "false");
