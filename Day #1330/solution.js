// Day 1330: Column number -> spreadsheet label (bijective base-26).
// Repeatedly take (n-1)%26 for the next letter, then n=(n-1)/26. O(log n) time.

function columnTitle(n) {
  let s = "";
  while (n > 0) {
    n--;
    s = String.fromCharCode(65 + (n % 26)) + s;
    n = Math.floor(n / 26);
  }
  return s;
}

console.log(`"${columnTitle(1)}"`);  // "A"
console.log(`"${columnTitle(27)}"`); // "AA"
