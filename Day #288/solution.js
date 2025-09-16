// Kaprekar's routine: repeatedly subtract ascending-digit from descending-digit
// (4-digit, zero-padded) until 6174; count steps. Time: O(7) iters, Space: O(1).
function kaprekar(n) {
  let steps = 0;
  while (n !== 6174) {
    const d = String(n).padStart(4, '0').split('');
    const asc = parseInt(d.slice().sort().join(''), 10);
    const desc = parseInt(d.slice().sort().reverse().join(''), 10);
    n = desc - asc;
    steps++;
  }
  return steps;
}

console.log(kaprekar(1234));
