// Day 1298: Count Kaprekar steps to reach 6174.
// Repeatedly sort digits desc - asc until 6174. Converges in <=7 steps. O(steps) time.
function kaprekarSteps(n) {
  let steps = 0;
  while (n !== 6174) {
    const s = String(n).padStart(4, "0");
    const asc = parseInt([...s].sort().join(""), 10);
    const desc = parseInt([...s].sort().reverse().join(""), 10);
    n = desc - asc;
    steps++;
    if (n === 0) break; // all digits equal
  }
  return steps;
}

console.log(kaprekarSteps(1234)); // 3
