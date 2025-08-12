// Day 108: B is a rotation of A iff |A|==|B| and B is a substring of A+A. O(n).
function isRotation(a, b) {
  return a.length === b.length && (a + a).includes(b);
}

console.log(isRotation("abcde", "cdeab"));
console.log(isRotation("abc", "acb"));
