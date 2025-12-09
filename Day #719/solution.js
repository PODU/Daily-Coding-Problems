// Day 719: Convert 1-based column number to spreadsheet id (bijective base-26).
// Repeatedly take (n-1)%26 then n=(n-1)/26. Time O(log n).
function colId(n) {
  let s = "";
  while (n > 0) {
    n--;
    s = String.fromCharCode(65 + (n % 26)) + s;
    n = Math.floor(n / 26);
  }
  return s;
}

console.log('"' + colId(1) + '"');
console.log('"' + colId(27) + '"');
