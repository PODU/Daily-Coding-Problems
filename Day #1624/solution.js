// Day 1624: Steps of Kaprekar's routine to reach 6174.
// Iterate sort-desc minus sort-asc until 6174. Time O(1) (bounded ~7 iters).
function kaprekarSteps(n) {
  let steps = 0;
  while (n !== 6174) {
    const s = String(n).padStart(4, "0").split("");
    const asc = parseInt(s.slice().sort().join(""), 10);
    const desc = parseInt(s.slice().sort().reverse().join(""), 10);
    n = desc - asc;
    steps++;
    if (n === 0) break;
  }
  return steps;
}

console.log(kaprekarSteps(1234));
