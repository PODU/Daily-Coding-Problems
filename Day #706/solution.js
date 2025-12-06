// Day 706: 24 Game (fixed order). Try every parenthesization over the fixed
// sequence, combining sub-results with +,-,*,/. Time ~O(1) for 4 numbers.
function solve(nums) {
  if (nums.length === 1) return [nums[0]];
  const res = [];
  for (let i = 1; i < nums.length; i++) {
    const lv = solve(nums.slice(0, i));
    const rv = solve(nums.slice(i));
    for (const a of lv) for (const b of rv) {
      res.push(a + b, a - b, a * b);
      if (Math.abs(b) > 1e-9) res.push(a / b);
    }
  }
  return res;
}

function game24(digits) {
  return solve(digits.map(Number)).some(v => Math.abs(v - 24.0) < 1e-6);
}

console.log(game24([5, 2, 7, 8]) ? "True" : "False");
