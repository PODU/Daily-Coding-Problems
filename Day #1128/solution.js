// All permutations of a list of digits in lexicographic order.
// Backtracking over sorted digits. O(n!*n) time, O(n) extra space.
function permutations(digits) {
  digits = [...digits].sort((a, b) => a - b);
  const n = digits.length;
  const used = new Array(n).fill(false);
  const cur = [];
  const res = [];

  function backtrack() {
    if (cur.length === n) {
      res.push([...cur]);
      return;
    }
    for (let i = 0; i < n; i++) {
      if (used[i]) continue;
      used[i] = true;
      cur.push(digits[i]);
      backtrack();
      cur.pop();
      used[i] = false;
    }
  }

  backtrack();
  return res;
}

const res = permutations([1, 2, 3]);
const inner = res.map((p) => "[" + p.join(",") + "]");
console.log("[" + inner.join(",") + "]");
