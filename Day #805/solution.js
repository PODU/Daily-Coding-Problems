// Day 805: Spreadsheet column number -> alphabetical label (bijective base 26).
// Repeatedly take (n-1)%26 for the letter, then n=(n-1)/26. Time O(log n), Space O(log n).

function columnLabel(n) {
  let s = "";
  while (n > 0) {
    n--;
    s = String.fromCharCode(65 + (n % 26)) + s;
    n = Math.floor(n / 26);
  }
  return s;
}

console.log(`"${columnLabel(1)}"`);  // "A"
console.log(`"${columnLabel(27)}"`); // "AA"
