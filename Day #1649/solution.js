// Bijective base-26: while n>0, n--, prepend 'A'+(n%26), n/=26. O(log n) time, O(log n) space.
function columnTitle(n) {
  let s = '';
  while (n > 0) {
    n--;
    s = String.fromCharCode(65 + (n % 26)) + s;
    n = Math.floor(n / 26);
  }
  return s;
}

console.log(columnTitle(1));
console.log(columnTitle(27));
