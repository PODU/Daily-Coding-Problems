// Phone keypad letter combinations via backtracking. O(prod of choices) time.

const M = {
  "2": "abc", "3": "def", "4": "ghi", "5": "jkl",
  "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz",
};

function letterCombinations(digits) {
  if (digits.length === 0) return [];
  const out = [];
  const cur = [];
  function backtrack(i) {
    if (i === digits.length) {
      out.push(cur.join(""));
      return;
    }
    for (const c of M[digits[i]]) {
      cur.push(c);
      backtrack(i + 1);
      cur.pop();
    }
  }
  backtrack(0);
  return out;
}

const res = letterCombinations("23");
console.log("[" + res.join(", ") + "]");
