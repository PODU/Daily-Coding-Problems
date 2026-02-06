// Day 1028: Kaprekar's routine. Repeatedly subtract ascending- from descending-
// digit arrangement until 6174; count steps. Time O(steps), Space O(1).
function kaprekarSteps(n) {
  let steps = 0;
  while (n !== 6174) {
    const s = String(n).padStart(4, "0").split("");
    const asc = parseInt(s.slice().sort().join(""), 10);
    const desc = parseInt(s.slice().sort().reverse().join(""), 10);
    n = desc - asc;
    steps++;
  }
  return steps;
}

console.log(kaprekarSteps(1234));
